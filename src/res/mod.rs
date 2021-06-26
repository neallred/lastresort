use actix_web::{HttpResponse};
use log::{error};

pub fn r400(x: &'static str) -> HttpResponse {
    HttpResponse::BadRequest().body(x)
}
//
// pub fn r404(x: &'static str) -> HttpResponse {
//     HttpResponse::NotFound().body(x)
// }
pub fn r500(x: &'static str) -> HttpResponse {
    error!("{}", x);
    HttpResponse::BadRequest().body(x)
}
