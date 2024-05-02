use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
// use tower_http::trace::TraceLayer;
// use tower_request_id::{RequestId, RequestIdLayer};
use tracing::Level;

#[macro_use]
extern crate tracing;

// use middleware::RegistryApplicationState;
// use tonic::transport::Server;

mod data_adapter;
mod encryption;
mod healthz;
mod middleware;
mod model;
mod protocol;
mod webhook;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Listening on port 8000");

    let routes = Router::new()
        .route("/", get(healthz::service_info))
        .route("/", post(webhook::target))
        .layer(axum::middleware::from_fn(middleware::log_request));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
    Ok(())
}
