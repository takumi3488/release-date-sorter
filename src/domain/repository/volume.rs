use anyhow::Result;

use crate::domain::entity::volume::{NewVolume, Volume};

#[async_trait::async_trait]
pub trait VolumeRepositoryTrait: Send + Sync + 'static {
    async fn insert(&self, volume: &NewVolume) -> Result<()>;
    async fn find_by_series_id(&self, series_id: &str) -> Result<Vec<Volume>>;
}
