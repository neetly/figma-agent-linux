use std::io;

use actix_cors::Cors;
use actix_web::{guard, middleware::Logger, web, App, HttpServer};
use env_logger::Env;

mod routes;

static ADDR: &str = "localhost:18412";
static ORIGIN: &str = "https://www.figma.com";

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allowed_origin(ORIGIN))
            .wrap(Logger::default())
            .service(
                web::scope("/figma")
                    .guard(guard::Header("Origin", ORIGIN))
                    .service(routes::font_files)
                    .service(routes::font_file),
            )
    })
    .bind(ADDR)?
    .run()
    .await
}
