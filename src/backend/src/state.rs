use crate::{config::AppCfg, infrastructure::db::Db};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: AppCfg,
    pub db: Db,
}

impl AppState {
    pub fn new(config: AppCfg, db: Db) -> Self {
        Self { config, db }
    }
}
