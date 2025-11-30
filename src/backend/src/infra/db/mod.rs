pub mod perms;
pub mod user;

use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::prelude::*;
use anyhow::Result;

pub async fn connect(database_url: &str) -> Result<PgPool> {
    debug!("open postgres pool started");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    debug!("open postgres pool completed");
    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> Result<()> {
    debug!("database migration started");
    sqlx::migrate!().run(pool).await?;
    debug!("database migration completed");
    Ok(())
}
