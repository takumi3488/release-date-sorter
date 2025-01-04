use anyhow::Result;

use crate::domain::{entity::series::Series, repository::series::SeriesRepositoryTrait};

#[derive(sqlx::FromRow)]
struct QuerySeriesResponse {
    id: String,
    url: String,
    title: String,
}

pub struct SeriesRepository {
    pub pool: sqlx::PgPool,
}

impl SeriesRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
impl SeriesRepositoryTrait for SeriesRepository {
    async fn find_by_id(&self, id: &str) -> Result<Series> {
        let series = sqlx::query_as!(
            QuerySeriesResponse,
            r#"
            SELECT id, url, title
            FROM series
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        let series = Series::new(&series.id, &series.url, &series.title);
        Ok(series)
    }

    async fn find_all(&self) -> Result<Vec<Series>> {
        let series = sqlx::query_as!(
            QuerySeriesResponse,
            r#"
            SELECT id, url, title
            FROM series
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        let series = series
            .into_iter()
            .map(|s| Series::new(&s.id, &s.url, &s.title))
            .collect();
        Ok(series)
    }
}
