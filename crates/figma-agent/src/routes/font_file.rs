use actix_web::{HttpResponse, Responder};

pub async fn font_file() -> impl Responder {
    HttpResponse::Ok()
}
