use std::process::exit;

use crate::prelude::*;

use sqlx::PgPool;

use crate::{config::AppCfg, domain::user::InternalNewUser, infra::db};

pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn new(config: &AppCfg) -> Self {
        debug!("Initiating new AppState");
        let db_pool = db::connect(&config.db_path)
            .await
            .map_err(|e| {
                error!("Failed to connect to database: {e}");
                exit(20);
            })
            .unwrap();

        db::migrate(&db_pool)
            .await
            .map_err(|e| {
                error!("Failed to migrade database: {e}");
                exit(22);
            })
            .unwrap();
        info!("DB connect and migrate sucessful");
        Self { db_pool: db_pool }
    }
}
