use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
use tracing::Level;

#[macro_use]
extern crate tracing;

mod config;
mod data_adapter;
mod healthz;
mod middleware;
mod webhook;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    config::validate();
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
