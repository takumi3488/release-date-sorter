use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::{
    config::Config,
    persistence::repository::{series::SeriesRepository, volume::VolumeRepository},
    usecase::pages::{danmachi::Danmachi, page::Page},
};

#[axum::debug_handler]
pub async fn crawl_pages(headers: HeaderMap, State(config): State<Config>) -> impl IntoResponse {
    let token = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok());
    if config.crawler_password.is_some_and(|password| {
        !token.is_some_and(|token| token == &format!("Bearer {}", password))
    }) {
        return (StatusCode::UNAUTHORIZED).into_response();
    }
    let series_repository = SeriesRepository::new(&config.db_pool);
    let volume_repository = VolumeRepository::new(&config.db_pool);
    tokio::spawn(async move {
        let _ = Danmachi::get_series(&series_repository, &volume_repository)
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
            });
    });
    (StatusCode::NO_CONTENT).into_response()
}
