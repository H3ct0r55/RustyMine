use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    domain::{supervisor::Supervisor, user},
    infrastructure::{
        config::AppCfg,
        db::Db,
        server_repo::{ServerRepo, SqliteServerRepo},
        user_repo::{SqliteUserRepo, UserRepo},
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppCfg,
    pub db: Db,
    pub supervisor: Supervisor,
    pub user_repo: Arc<dyn UserRepo + Send + Sync>,
    pub server_repo: Arc<dyn ServerRepo + Send + Sync>,
}

impl AppState {
    pub async fn new(config: &AppCfg) -> Self {
        let db = Db::connect_and_migrate(config).await.unwrap();
        let user_repo = Arc::new(SqliteUserRepo::new(db.pool.clone()));
        let server_repo = Arc::new(SqliteServerRepo::new(db.pool.clone()));
        Self {
            config: config.clone(),
            db,
            supervisor: Supervisor::new(),
            user_repo,
            server_repo,
        }
    }
}
