use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    config::Config,
    persistence::repository::{series::SeriesRepository, volume::VolumeRepository},
    usecase::pages::{danmachi::Danmachi, page::Page},
};

pub async fn crawl_pages(State(config): State<Config>) -> impl IntoResponse {
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
