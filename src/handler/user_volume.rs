use axum::{
    extract::{Path, State},
    http,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    config::Config,
    domain::{entity::user_volume::UserVolume, error::app::AppError},
    persistence::repository::{
        series::SeriesRepository, user_volume::UserVolumeRepository, volume::VolumeRepository,
    },
    usecase::{series::SeriesUseCase, user_volume::UserVolumeUseCase},
};

#[derive(Serialize)]
pub struct GetSeriesWithCheckingResponse {
    pub id: String,
    pub url: String,
    pub title: String,
    pub volumes: Vec<VolumeWithChecking>,
}

#[derive(Serialize)]
pub struct VolumeWithChecking {
    pub id: Uuid,
    pub title: String,
    pub publication_date: chrono::NaiveDate,
    pub checked: bool,
}

pub async fn get_series_with_checking(
    State(config): State<Config>,
    Path((user_id, series_id)): Path<(Uuid, String)>,
) -> Result<Json<GetSeriesWithCheckingResponse>, AppError> {
    let series_repository = SeriesRepository::new(&config.db_pool);
    let volume_repository = VolumeRepository::new(&config.db_pool);
    let user_volume_repository = UserVolumeRepository::new(&config.db_pool);
    let series_usecase =
        SeriesUseCase::new(Box::new(series_repository), Box::new(volume_repository));
    let user_volume_usecase = UserVolumeUseCase::new(Box::new(user_volume_repository));
    let series = series_usecase.get_by_id(&series_id).await?;
    let user_volumes = user_volume_usecase.get_by_user_id(&user_id).await?;

    let volumes = series
        .volumes
        .iter()
        .map(|volume| {
            let checked = user_volumes
                .iter()
                .any(|user_volume| user_volume.volume_id == volume.id && user_volume.checked);
            VolumeWithChecking {
                id: volume.id,
                title: volume.title.clone(),
                publication_date: volume.publication_date,
                checked,
            }
        })
        .collect();
    Ok(Json(GetSeriesWithCheckingResponse {
        id: series.id.clone(),
        url: series.url.clone(),
        title: series.title.clone(),
        volumes,
    }))
}

pub async fn upsert_user_volume(
    State(config): State<Config>,
    Json(user_volume): Json<UserVolume>,
) -> impl IntoResponse {
    let user_volume_repository = UserVolumeRepository::new(&config.db_pool);
    let user_volume_usecase = UserVolumeUseCase::new(Box::new(user_volume_repository));
    if let Err(e) = user_volume_usecase.upsert(&user_volume).await {
        (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    } else {
        http::StatusCode::NO_CONTENT.into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures("init"))]
    #[ignore]
    async fn test_user_volume(pool: sqlx::PgPool) {
        let config = Config { db_pool: pool };

        // 初期状態では何もない
        let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let series_id = "series1".to_string();
        let response =
            get_series_with_checking(State(config.clone()), Path((user_id, series_id.clone())))
                .await
                .unwrap();
        assert_eq!(response.id, "series1");
        assert_eq!(response.volumes.iter().filter(|v| v.checked).count(), 0);

        // チェックが追加できる
        let user_volume = UserVolume {
            user_id,
            volume_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            checked: true,
        };
        upsert_user_volume(State(config.clone()), Json(user_volume)).await;
        let response =
            get_series_with_checking(State(config.clone()), Path((user_id, series_id.clone())))
                .await
                .unwrap();
        assert_eq!(response.id, "series1");
        assert_eq!(response.volumes.iter().filter(|v| v.checked).count(), 1);

        // チェックが更新できる
        let user_volume = UserVolume {
            user_id,
            volume_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            checked: false,
        };
        let response = upsert_user_volume(State(config.clone()), Json(user_volume)).await;
        assert_eq!(
            response.into_response().status(),
            http::StatusCode::NO_CONTENT
        );
        let response =
            get_series_with_checking(State(config.clone()), Path((user_id, series_id.clone())))
                .await
                .unwrap();
        assert_eq!(response.id, "series1");
        assert_eq!(response.volumes.iter().filter(|v| v.checked).count(), 0);
    }
}
