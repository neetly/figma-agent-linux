use std::sync::LazyLock;

use axum::{Router, http::HeaderValue, routing::get};
use figma_agent::{CONFIG, FONT_FILES, routes};
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    LazyLock::force(&CONFIG);
    LazyLock::force(&FONT_FILES);

    let app = Router::new()
        .route("/figma/font-files", get(routes::font_files))
        .route("/figma/font-file", get(routes::font_file))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(HeaderValue::from_static("https://www.figma.com"))
                        .allow_private_network(true),
                ),
        );

    let listener = match ListenFd::from_env().take_tcp_listener(0)? {
        Some(listener) => {
            listener.set_nonblocking(true)?;
            TcpListener::from_std(listener)?
        }
        None => TcpListener::bind(&CONFIG.load().bind).await?,
    };
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
