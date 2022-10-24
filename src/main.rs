use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Set the logging environment to debug if not set
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "ping_api=debug,tower_http=debug");
    }
    tracing_subscriber::fmt::init();
    // Create the routes and add in our queue data
    let routes = Router::new()
        .route("/", get(ping))
        .route("/ping", get(ping))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    // Start the server
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> Result<String, ()> {
    Ok("pong".to_string())
}
