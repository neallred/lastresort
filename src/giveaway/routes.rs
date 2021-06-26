use crate::giveaway::{CreateRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::res;

#[post("/giveaways")]
async fn create(giveaway: web::Json<CreateRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = create(db_pool.get_ref(), giveaway.into_inner()).await;
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        _ => res::r500("Error trying to create new giveaway")
    }
}

// #[get("/giveaways")]
// async fn list(db_pool: web::Data<PgPool>) -> impl Responder {
//     let result = ::list(db_pool.get_ref()).await;
//     match result {
//         Ok(x) => HttpResponse::Ok().json(x),
//         _ => res::r500("Error trying to fetch giveaways")
//     }
// }

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
//    cfg.service(list);
}
