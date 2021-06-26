use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow, Row};
use sqlx::postgres::PgRow;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatePicture {
    pub bytes: Vec<u8>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub pictures: Vec<CreatePicture>,
}

pub async fn create(pool: &PgPool, request: CreateRequest) -> Result<i64> {
    let mut tx = pool.begin().await?;
    let now = Utc::now();

    let giveaway_id: i64 = sqlx::query("INSERT INTO giveaways (user_id, name, description, updated) VALUES ($1, $2, $3, $4) RETURNING id")
        .bind(&request.user_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&now)
        .map(|row: PgRow| row.get(0))
        .fetch_one(&mut tx)
        .await?;

    for p in &request.pictures {
        sqlx::query("INSERT INTO pictures (giveaway_id, name, bytes, description) VALUES ($1, $2, $3, $4)")
        .bind(&giveaway_id)
        .bind(&p.bytes)
        .bind(&p.description)
        .await?;
    }

    tx.commit().await?;

    Ok(giveaway_id)
}
