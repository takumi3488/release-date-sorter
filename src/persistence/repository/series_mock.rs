use anyhow::Result;

use crate::domain::{entity::series::Series, repository::series::SeriesRepositoryTrait};

pub struct SeriesRepositoryMock {
    pub series: Vec<Series>,
}

#[async_trait::async_trait]
impl SeriesRepositoryTrait for SeriesRepositoryMock {
    async fn find_by_id(&self, id: &str) -> Result<Series> {
        Ok(self.series.iter().find(|s| s.id == id).unwrap().clone())
    }

    async fn find_all(&self) -> Result<Vec<Series>> {
        Ok(self.series.clone())
    }
}
