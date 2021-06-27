use actix_web::{web, App, HttpServer, middleware};
use actix_files::Files;
// use actix_session::{CookieSession, Session};
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::migrate::{MigrateDatabase};
use anyhow::Result;
use env_logger::Env;

use std::env;
use dotenv::dotenv;

mod res;
mod user;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let database_url = env::var("LASTRESORT_DATABASE_URL").expect("LASTRESORT_DATABASE_URL not in .env file");
    let host = env::var("LASTRESORT_HOST").expect("LASTRESORT_HOST not in .env file");
    let port = env::var("LASTRESORT_PORT").expect("LASTRESORT_PORT not in .env file");
    let static_dir = env::var("LASTRESORT_STATIC_DIR").expect("LASTRESORT_STATIC_DIR not in .env file");

    let db_exists: bool = Postgres::database_exists(&database_url).await?;
    if !db_exists {
        println!("db duz not exist");
        Postgres::create_database(&database_url).await?;
    }

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    let _ = user::bootstrap_owner(&db_pool).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/api")
                .data(db_pool.clone())
                .configure(user::init)
            )
            .service(Files::new("/", &static_dir).index_file("index.html").prefer_utf8(true))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    Ok(())
}
