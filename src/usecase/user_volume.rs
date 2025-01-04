use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entity::user_volume::UserVolume, repository::user_volume::UserVolumeRepositoryTrait,
};

pub struct UserVolumeUseCase {
    user_volume_repository: Box<dyn UserVolumeRepositoryTrait>,
}

impl UserVolumeUseCase {
    pub fn new(user_volume_repository: Box<dyn UserVolumeRepositoryTrait>) -> Self {
        Self {
            user_volume_repository,
        }
    }

    pub async fn get_by_user_id(&self, user_id: &Uuid) -> Result<Vec<UserVolume>> {
        self.user_volume_repository.find_by_user_id(user_id).await
    }

    pub async fn upsert(&self, user_volume: &UserVolume) -> Result<()> {
        self.user_volume_repository.upsert(user_volume).await
    }
}
