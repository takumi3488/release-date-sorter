use anyhow::Result;

use crate::domain::{
    entity::series::Series,
    repository::{series::SeriesRepositoryTrait, volume::VolumeRepositoryTrait},
};

pub struct SeriesUseCase {
    series_repository: Box<dyn SeriesRepositoryTrait>,
    volume_repository: Box<dyn VolumeRepositoryTrait>,
}

impl SeriesUseCase {
    pub fn new(
        series_repository: Box<dyn SeriesRepositoryTrait>,
        volume_repository: Box<dyn VolumeRepositoryTrait>,
    ) -> Self {
        Self {
            series_repository,
            volume_repository,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Series>> {
        self.series_repository.find_all().await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Series> {
        let series = self.series_repository.find_by_id(id).await?;
        let volumes = self.volume_repository.find_by_series_id(id).await?;
        let mut series = Series::new(&series.id, &series.url, &series.title).with_volumes(volumes);
        series.sort_volumes_by_publication_date();
        Ok(series)
    }
}
