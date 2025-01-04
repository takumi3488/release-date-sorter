use anyhow::Result;

use crate::domain::{
    entity::volume::{NewVolume, Volume},
    repository::volume::VolumeRepositoryTrait,
};

pub struct VolumeRepository {
    pub pool: sqlx::PgPool,
}

impl VolumeRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
impl VolumeRepositoryTrait for VolumeRepository {
    async fn insert(&self, volume: &NewVolume) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO volumes (series_id, title, publication_date)
            VALUES ($1, $2, $3)
            "#,
            volume.series_id,
            volume.title,
            volume.publication_date
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_series_id(&self, series_id: &str) -> Result<Vec<Volume>> {
        let volumes = sqlx::query_as!(
            Volume,
            r#"
            SELECT id, title, publication_date
            FROM volumes
            WHERE series_id = $1
            "#,
            series_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(volumes)
    }
}
