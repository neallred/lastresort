use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow, Row};
use sqlx::postgres::PgRow;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use hex;
use sha3::{Digest, Sha3_512};
use log::{info};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub username: String,
    pub password: String, // unencrypted
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String, // encrypted
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDisplay {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub issued: DateTime<Utc>,
}

fn make_token() -> Result<String> {
    Ok(format!("{}", Uuid::new_v4()))
}

pub fn hash_pw(passwd: &str) -> String {
    let mut hasher = Sha3_512::new();

    hasher.update(passwd.as_bytes());

    let result = hasher.finalize();
    hex::encode(result)
}

pub async fn bootstrap_owner(pool: &PgPool) -> Result<Option<String>> {
    info!("checking if need to bootstrap owner");
    let has_owner = sqlx::query!(
        r#"
        SELECT COUNT(users.id) FROM users LEFT JOIN user_roles ON users.id = user_roles.user_id LEFT JOIN roles ON user_roles.role_id = roles.id WHERE roles.name = 'owner' LIMIT 1;
        "#
        )
        .fetch_optional(pool)
        .await?;

    if has_owner.is_none() {
        let claim_owner_password = format!("{}", Uuid::new_v4());
        println!("needs an owner, claim ownership by making user with password or by changing password on existing user to {}", &claim_owner_password);
        return Ok(Some(claim_owner_password));
    } else {
        println!("has owner, nothing to do");
        return Ok(None)
    }
}


impl User {
    pub async fn exists(pool: &PgPool, username: &String) -> Result<bool> {
        let user = sqlx::query!(
            r#"
                SELECT username
                    FROM users
                WHERE username = $1
            "#,
            username
            )
            .fetch_optional(pool)
            .await?;

        Ok(user.is_some())
    }

    pub async fn validate_pw(pool: &PgPool, username: &String, pw: &str) -> (i64, bool) {
        if let Ok(result) = sqlx::query!(
            r#"
                SELECT id, password
                    FROM users
                WHERE username = $1
            "#,
            username
        )
            .fetch_one(pool)
            .await 
        {
            return (result.id, hash_pw(pw) == result.password)
        };
        return (-1, false)
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<UserDisplay>> {
        let mut xs = vec![];
        let users = sqlx::query!("SELECT id, username FROM users")
            .fetch_all(pool)
            .await?;
        for u in users {
            xs.push(UserDisplay {
                id: u.id,
                username: u.username,
            });
        }

        Ok(xs)
    }

    pub async fn create(pool: &PgPool, request: CreateRequest) -> Result<String> {
        let mut tx = pool.begin().await?;
        if User::exists(pool, &request.username).await? {
            return Err(anyhow!("Can not create user {} . That user already exists.", request.username))
        }
        let new_user_token = make_token()?;
        let hashed = hash_pw(&request.password);
        let now = Utc::now();

        let user_id: i64 = sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id")
            .bind(&request.username)
            .bind(&hashed)
            .map(|row: PgRow| row.get(0))
            .fetch_one(&mut tx)
            .await?;

        let _ = sqlx::query("INSERT INTO tokens (token, user_id, issued) VALUES ($1, $2, $3) RETURNING id")
            .bind(&new_user_token)
            .bind(user_id)
            .bind(now)
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;

        Ok(new_user_token)
    }

    pub async fn login(pool: &PgPool, request: CreateRequest) -> Result<String> {
        let (user_id, is_valid) = User::validate_pw(pool, &request.username, &request.password).await;
        if !is_valid {
            return Err(anyhow!("Incorrect username or password for {}", request.username))
        }

        let new_user_token = make_token()?;
        let now = Utc::now();

        let mut tx = pool.begin().await?;
        let _ = sqlx::query("INSERT INTO tokens (token, user_id, issued) VALUES ($1, $2, $3) RETURNING id")
            .bind(&new_user_token)
            .bind(user_id)
            .bind(now)
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;

        Ok(new_user_token)
    }

    pub async fn logout(pool: &PgPool, token: String) -> Result<()> {
        let _ = sqlx::query("DELETE FROM tokens WHERE token = $1")
            .bind(token)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn is_admin(pool: &PgPool, token: String) -> Result<bool> {
        if let Ok(result) = sqlx::query!("SELECT user_id FROM tokens WHERE token = $1", token).fetch_one(pool).await
        {
            if let Ok(result) = sqlx::query!("SELECT role_id, group_id FROM user_roles WHERE user_id = $1", result.user_id).fetch_one(pool).await {
                if let Ok(result) = sqlx::query!("SELECT name FROM roles WHERE id = $1", result.role_id).fetch_one(pool).await {
                    if result.name == "admin" || result.name == "owner" {
                        return Ok(true)
                    }
                }
            }
        }
        return Ok(false)
    }

    pub async fn delete(pool: &PgPool, token: String, id_to_delete: i64) -> Result<()> {
        if let Ok(true) = User::is_admin(pool, token).await {
            let _ = sqlx::query!("DELETE FROM users WHERE id = $1", id_to_delete)
                .execute(pool)
                .await?;

            return Ok(());
        }
        Err(anyhow!("Unable to delete user, could not validate that this token belongs to an admin"))
    }

    // pub async fn is_logged_in(pool: &PgPool, id: i64, token: String) -> Result<bool> {
    //     let user_token = sqlx::query!(
    //         r#"
    //             SELECT id, token
    //                 FROM tokens
    //             WHERE id = $1 AND token = $2
    //         "#,
    //         id,
    //         token
    //         )
    //         .fetch_optional(pool)
    //         .await?;

    //     Ok(user_token.is_some())
    // }
}
