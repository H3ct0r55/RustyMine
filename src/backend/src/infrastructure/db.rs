use crate::infrastructure::config::AppCfg;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::Path;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn connect_and_migrate(config: &AppCfg) -> Result<Db> {
        let path = Path::new(&config.master_path).join(&config.db_path);
        let url = format!("sqlite:{}", path.to_str().unwrap());
        info!("Connecting to {}", url);
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;
        info!("Database connection ready, all migrations completed");
        Ok(Db { pool })
    }
}
