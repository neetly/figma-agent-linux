use actix_web::{HttpResponse, Responder};

pub async fn font_files() -> impl Responder {
    HttpResponse::Ok()
}
