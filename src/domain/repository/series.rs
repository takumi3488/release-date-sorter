use anyhow::Result;

use crate::domain::entity::series::Series;

#[async_trait::async_trait]
pub trait SeriesRepositoryTrait: Send + Sync + 'static {
    async fn find_by_id(&self, id: &str) -> Result<Series>;
    async fn find_all(&self) -> Result<Vec<Series>>;
}
