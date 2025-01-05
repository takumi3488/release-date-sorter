use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_pool: sqlx::PgPool,
    pub crawler_password: Option<String>,
}

impl Config {
    pub async fn load() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();
        let crawler_password = env::var("CRAWLER_PASSWORD").ok();
        let db_pool = sqlx::PgPool::connect(&database_url).await.unwrap();
        Self {
            db_pool,
            crawler_password,
        }
    }
}
