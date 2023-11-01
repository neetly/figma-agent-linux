use std::io;

use actix_cors::Cors;
use actix_web::{guard, middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use listenfd::ListenFd;

mod payload;
mod routes;

static ADDR: &str = "localhost:18412";
static ORIGIN: &str = "https://www.figma.com";

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut listen_fd = ListenFd::from_env();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET"])
                    .allowed_origin(ORIGIN)
                    .allow_private_network_access(),
            )
            .service(
                web::scope("/figma")
                    .guard(guard::Header("Origin", ORIGIN))
                    .service(routes::font_files)
                    .service(routes::font_file),
            )
    });

    let server = if let Some(listener) = listen_fd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(ADDR)?
    };

    server.workers(1).run().await
}
