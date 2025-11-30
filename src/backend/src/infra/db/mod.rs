use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use anyhow::Result;

pub async fn connect(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> Result<()> {
    sqlx::migrate!().run(&pool).await?;
    Ok(())
}
