use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    config::Config,
    persistence::repository::{series::SeriesRepository, volume::VolumeRepository},
    usecase::series::SeriesUseCase,
};

pub async fn get_series(State(config): State<Config>) -> impl IntoResponse {
    let series_repository = SeriesRepository::new(&config.db_pool);
    let volume_repository = VolumeRepository::new(&config.db_pool);
    let usecase = SeriesUseCase::new(Box::new(series_repository), Box::new(volume_repository));
    if let Ok(series) = usecase.get_all().await {
        (StatusCode::OK, Json(series)).into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Internal Server Error"),
        )
            .into_response()
    }
}

pub async fn get_series_by_id(
    State(config): State<Config>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let series_repository = SeriesRepository::new(&config.db_pool);
    let volume_repository = VolumeRepository::new(&config.db_pool);
    let usecase = SeriesUseCase::new(Box::new(series_repository), Box::new(volume_repository));
    if let Ok(series) = usecase.get_by_id(&id).await {
        (StatusCode::OK, Json(series)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Series not found").into_response()
    }
}
