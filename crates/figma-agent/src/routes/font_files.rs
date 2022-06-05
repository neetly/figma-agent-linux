use actix_web::{get, HttpResponse, Responder};

#[get("/font-files")]
pub async fn font_files() -> impl Responder {
    HttpResponse::Ok()
}
