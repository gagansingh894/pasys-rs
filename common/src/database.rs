use anyhow;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub struct Config {
    pub reader_url: String,
    pub reader_max_connections: u32,
    pub writer_url: String,
    pub writer_max_connections: u32,
    pub timeout_in_secs: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub reader: PgPool,
    pub writer: PgPool,
}

impl Database {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let reader_pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(config.timeout_in_secs))
            .max_connections(config.reader_max_connections)
            .connect(&config.reader_url)
            .await?;

        let writer_pool = PgPoolOptions::new()
            .max_connections(config.writer_max_connections)
            .connect(&config.writer_url)
            .await?;

        Ok(Self {
            reader: reader_pool,
            writer: writer_pool,
        })
    }

    pub async fn from_pool(pool: PgPool) -> anyhow::Result<Self> {
        Ok(Self {
            reader: pool.clone(),
            writer: pool,
        })
    }
}
