use crate::{config::AppCfg, domain::repository::UserRepository};
use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use tracing::info;

#[derive(Clone, Debug)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn connect_and_migrate(config: &AppCfg) -> Result<Db> {
        let url = format!("sqlite:{}", config.db_path);
        info!("Connecting to {}", url);
        let options = SqliteConnectOptions::new()
            .filename(&config.db_path)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;
        info!("Database connection ready, all migrations completed");
        Ok(Db { pool })
    }
}
