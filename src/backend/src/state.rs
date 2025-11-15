use std::sync::Arc;

use crate::{
    config::AppCfg,
    domain::repository::{ServerRepository, UserRepository},
    infrastructure::{
        db::Db, sqlite_server_repo::SqliteServerRepository, sqlite_user_repo::SqliteUserRepository,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppCfg,
    pub db: Db,
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
    pub server_repo: Arc<dyn ServerRepository + Send + Sync>,
}

impl AppState {
    pub fn new(config: AppCfg, db: Db) -> Self {
        let user_repo = Arc::new(SqliteUserRepository::new(db.pool.clone()));
        let server_repo = Arc::new(SqliteServerRepository::new(db.pool.clone()));
        Self {
            config,
            db,
            user_repo,
            server_repo,
        }
    }
}
