use crate::user::{User, CreateRequest};
use actix_web::{get, delete, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::res;

#[post("/users")]
async fn create(user: web::Json<CreateRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(db_pool.get_ref(), user.into_inner()).await;
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        _ => res::r500("Error trying to create new user")
    }
}

#[post("/users/login")]
async fn login(user: web::Json<CreateRequest>, db_pool: web::Data<PgPool>) -> impl Responder {

    let result = User::login(db_pool.get_ref(), user.into_inner()).await;
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        _ => res::r400("Unable to log in")
    }
}

#[delete("/users/login")]
async fn logout(token: String, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::logout(db_pool.get_ref(), token).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(true),
        _ => res::r400("Unable to log out")
    }
}

#[get("/users")]
async fn list(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::list(db_pool.get_ref()).await;
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        _ => res::r500("Error trying to create new user")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(list);
    cfg.service(login);
    cfg.service(logout);
}
