use anyhow::Result;
use uuid::Uuid;

use crate::domain::entity::user_volume::UserVolume;

#[async_trait::async_trait]
pub trait UserVolumeRepositoryTrait: Send + Sync {
    async fn upsert(&self, user_volumes: &UserVolume) -> Result<()>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<UserVolume>>;
}
