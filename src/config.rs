use std::env;

use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_pool: sqlx::PgPool,
    pub crawler_password: Option<String>,
}

impl Config {
    pub async fn load() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")?;
        let crawler_password = env::var("CRAWLER_PASSWORD").ok();
        let db_pool = sqlx::PgPool::connect(&database_url).await?;
        Ok(Self {
            db_pool,
            crawler_password,
        })
    }
}
