use anyhow::Result;
use scraper::Html;

use crate::domain::{
    entity::{series::Series, volume::NewVolume},
    repository::{series::SeriesRepositoryTrait, volume::VolumeRepositoryTrait},
};

pub trait Page<T> {
    async fn get_html_from_url() -> Result<Html> {
        let document = reqwest::get(Self::get_url()).await?.text().await?;
        Ok(Self::get_html_from_document(&document))
    }
    fn get_html_from_document(document: &str) -> Html {
        Html::parse_document(document)
    }
    fn get_volumes(html: Html) -> Vec<NewVolume>;
    fn get_url() -> String;
    async fn get_series(
        series_repository: &impl SeriesRepositoryTrait,
        volume_repository: &impl VolumeRepositoryTrait,
    ) -> Result<Series> {
        let html = Self::get_html_from_url().await?;
        let volumes = Self::get_volumes(html);
        let series = series_repository.find_by_id(&Self::get_url()).await?;
        for volume in &volumes {
            if volume_repository.insert(volume).await.is_err_and(|e| {
                !e.to_string()
                    .contains("duplicate key value violates unique constraint")
            }) {
                return Err(anyhow::anyhow!("Failed to insert volume"));
            }
        }
        let volumes = volume_repository.find_by_series_id(&series.id).await?;
        let mut series =
            Series::new(&series.id, &Self::get_url(), &series.title).with_volumes(volumes);
        series.sort_volumes_by_publication_date();
        Ok(series)
    }
}
