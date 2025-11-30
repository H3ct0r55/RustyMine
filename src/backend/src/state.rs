use std::process::exit;

use crate::prelude::*;

use sqlx::PgPool;

use crate::{config::AppCfg, infra::db};

pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn new(config: &AppCfg) -> Self {
        debug!("init app state");
        debug!("establish database connection");
        let db_pool = db::connect(&config.db_path)
            .await
            .map_err(|e| {
                error!(error = %e, "connect to database failed");
                exit(20);
            })
            .unwrap();

        debug!("run database migrations");
        db::migrate(&db_pool)
            .await
            .map_err(|e| {
                error!(error = %e, "database migration failed");
                exit(22);
            })
            .unwrap();
        info!("database ready after connect and migrate");
        Self { db_pool: db_pool }
    }
}
