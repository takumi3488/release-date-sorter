use std::env;

#[derive(Clone)]
pub struct Config {
    pub db_pool: sqlx::PgPool,
}

impl Config {
    pub async fn load() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();
        let db_pool = sqlx::PgPool::connect(&database_url).await.unwrap();
        Self { db_pool }
    }
}
