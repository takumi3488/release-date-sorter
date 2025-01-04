mod config;
mod domain;
mod handler;
mod persistence;
mod usecase;
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use handler::{
    crawler::crawl_pages,
    series::{get_series, get_series_by_id},
    user_volume::{get_series_with_user_volumes, upsert_user_volume},
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::debug;

fn app(config: Config) -> Router {
    let static_dir = ServeDir::new("dist");
    let api_routes = Router::new()
        .route("/health", get(health))
        .route("/crawl", post(crawl_pages))
        .route("/series", get(get_series))
        .route("/series/{id}", get(get_series_by_id))
        .route(
            "/{user_id}/series/{series_id}",
            get(get_series_with_user_volumes),
        )
        .route("/user_volumes", post(upsert_user_volume));
    Router::new()
        .fallback_service(static_dir)
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let config = Config::load().await;
    let app = app(config);
    axum::serve(listener, app).await?;
    Ok(())
}

#[tracing::instrument]
async fn health() -> String {
    debug!("health check");
    "OK".to_string()
}
