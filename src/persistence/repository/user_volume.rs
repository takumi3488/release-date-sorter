use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entity::user_volume::UserVolume, repository::user_volume::UserVolumeRepositoryTrait,
};

pub struct UserVolumeRepository {
    pub pool: sqlx::PgPool,
}

impl UserVolumeRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
impl UserVolumeRepositoryTrait for UserVolumeRepository {
    async fn upsert(&self, user_volume: &UserVolume) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_volumes (user_id, volume_id, checked)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, volume_id) DO UPDATE
            SET checked = $3
            "#,
            user_volume.user_id,
            user_volume.volume_id,
            user_volume.checked
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<UserVolume>> {
        let user_volumes = sqlx::query_as!(
            UserVolume,
            r#"
            SELECT
                user_id,
                volume_id,
                checked
            FROM user_volumes
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_volumes)
    }
}
