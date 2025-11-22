use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sqlx::SqlitePool;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;

use regex::Regex;

use crate::{
    domain::{supervisor::Supervisor, user},
    infrastructure::{
        config::AppCfg,
        db::Db,
        server_repo::{ServerRepo, SqliteServerRepo},
        user_repo::{SqliteUserRepo, UserRepo},
    },
    utils::validation::load_valid_versions,
};

#[derive(Clone)]
pub struct VersionManifest {
    pub manifest: HashSet<String>,
    pub last_updated: DateTime<Utc>,
}

impl VersionManifest {
    pub async fn new() -> Self {
        Self {
            manifest: load_valid_versions().await.unwrap(),
            last_updated: Utc::now(),
        }
    }

    fn needs_refresh(&self) -> bool {
        let age = Utc::now().signed_duration_since(self.last_updated);
        age > Duration::hours(6)
    }

    pub async fn validate(&mut self, input: &str) -> Result<()> {
        let re =
            Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)(?:\.(0|[1-9]\d*))?$").expect("invalid regex");

        if !re.is_match(input) {
            return Err(anyhow!("Invalid Minecraft version format: {}", input));
        }
        if self.needs_refresh() {
            self.manifest = load_valid_versions().await.unwrap();
            self.last_updated = Utc::now();
        }

        if !self.manifest.contains(input) {
            return Err(anyhow!("Invalid minecraft version: {}", input));
        }
        Ok(())
    }
}

pub struct AppState {
    pub config: AppCfg,
    pub db: Db,
    pub supervisor: Supervisor,
    pub user_repo: Arc<dyn UserRepo + Send + Sync>,
    pub server_repo: Arc<dyn ServerRepo + Send + Sync>,
    pub version_manifest: RwLock<VersionManifest>,
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
            version_manifest: RwLock::new(VersionManifest::new().await),
        }
    }
}
