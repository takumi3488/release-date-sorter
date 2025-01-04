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
    domain::{
        entity::{series::Series, user_volume::UserVolume},
        error::app::AppError,
    },
    persistence::repository::{
        series::SeriesRepository, user_volume::UserVolumeRepository, volume::VolumeRepository,
    },
    usecase::{series::SeriesUseCase, user_volume::UserVolumeUseCase},
};

#[derive(Serialize)]
pub struct GetSeriesWithUserVolumesResponse {
    series: Series,
    user_volumes: Vec<UserVolume>,
}

pub async fn get_series_with_user_volumes(
    State(config): State<Config>,
    Path((user_id, series_id)): Path<(Uuid, String)>,
) -> Result<Json<GetSeriesWithUserVolumesResponse>, AppError> {
    let series_repository = SeriesRepository::new(&config.db_pool);
    let volume_repository = VolumeRepository::new(&config.db_pool);
    let user_volume_repository = UserVolumeRepository::new(&config.db_pool);
    let series_usecase =
        SeriesUseCase::new(Box::new(series_repository), Box::new(volume_repository));
    let user_volume_usecase = UserVolumeUseCase::new(Box::new(user_volume_repository));
    let series = series_usecase.get_by_id(&series_id).await?;
    let user_volumes = user_volume_usecase.get_by_user_id(&user_id).await?;

    Ok(Json(GetSeriesWithUserVolumesResponse {
        series,
        user_volumes,
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
            get_series_with_user_volumes(State(config.clone()), Path((user_id, series_id.clone())))
                .await
                .unwrap();
        assert_eq!(response.series.id, "series1");
        assert_eq!(response.user_volumes.len(), 0);

        // チェックが追加できる
        let user_volume = UserVolume {
            user_id,
            volume_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            checked: true,
        };
        upsert_user_volume(State(config.clone()), Json(user_volume)).await;
        let response =
            get_series_with_user_volumes(State(config.clone()), Path((user_id, series_id.clone())))
                .await
                .unwrap();
        assert_eq!(response.series.id, "series1");
        assert_eq!(response.user_volumes.len(), 1);
        assert_eq!(
            response.user_volumes[0].volume_id,
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
        );
        assert!(response.user_volumes[0].checked);

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
            get_series_with_user_volumes(State(config.clone()), Path((user_id, series_id.clone())))
                .await
                .unwrap();
        assert_eq!(response.series.id, "series1");
        assert_eq!(response.user_volumes.len(), 1);
        assert_eq!(
            response.user_volumes[0].volume_id,
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
        );
        assert!(!response.user_volumes[0].checked);
    }
}
