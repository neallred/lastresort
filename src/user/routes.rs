use crate::user::{User, CreateRequest};
use actix_web::{get, delete, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use crate::res;


#[post("/users")]
async fn create(user: web::Json<CreateRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(db_pool.get_ref(), user.into_inner()).await;
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(x) => HttpResponse::InternalServerError().body(format!("Error trying to create user: {:?}", x)),
    }
}

fn get_bearer_token(req: &HttpRequest) -> Option<String> {
    let headers = req.headers();
    if let Some(auth_header) = headers.get("Authentication") {
        if let Ok(header_str) = auth_header.to_str() {
            if header_str.starts_with("Bearer ") {
                return Some(header_str.trim_start_matches("Bearer ").to_string())
            }
        }
    }
    None
}

#[delete("/users/{id}")]
async fn delete(req: HttpRequest, id: web::Path<i64>, db_pool: web::Data<PgPool>) -> impl Responder {
    if let Some(token) = get_bearer_token(&req) {
        let result = User::delete(db_pool.get_ref(), token, *id).await;
        return match result {
            Ok(x) => HttpResponse::Ok().json(x),
            Err(x) => HttpResponse::InternalServerError().body(format!("Error trying to delete user: {:?}", x)),
        }
    };
    res::r403("Unauthorized")
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
    cfg.service(delete);
    cfg.service(logout);
}
