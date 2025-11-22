use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{domain::user::User, infrastructure::state::AppState};

#[async_trait]
pub trait UserRepo: Send + Sync {
    //async fn get_by_id(&self, id: i64) -> Result<Option<User>>;
}

pub struct SqliteUserRepo {
    pool: SqlitePool,
}

impl SqliteUserRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for SqliteUserRepo {}
