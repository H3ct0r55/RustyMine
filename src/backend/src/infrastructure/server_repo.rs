use std::{os::linux::raw::stat, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;
use tracing::{error, info};

use crate::{
    domain::server::{NewServer, NewServerRow, Server, ServerRow},
    infrastructure::state::AppState,
};

#[async_trait]
pub trait ServerRepo: Send + Sync {
    async fn get_by_id(&self, id: i64) -> Result<Option<Server>>;
    //async fn list_all(&self) -> Result<Vec<Server>>;
    async fn create(&self, new_server: NewServer) -> Result<Server>;
    //async fn disable(&self, id: i64) -> Result<()>;
    //async fn delete(&self, id: i64) -> Result<()>;
}

pub struct SqliteServerRepo {
    pool: SqlitePool,
}

impl SqliteServerRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServerRepo for SqliteServerRepo {
    async fn get_by_id(&self, id: i64) -> Result<Option<Server>> {
        let row: Option<ServerRow> = sqlx::query_as::<_, ServerRow>(
            r#"
           SELECT
            id,
            name, 
            slug,
            is_active,
            path,
            jar_path,
            j_max_mem,
            j_min_mem,
            mc_type,
            mc_version,
            created_at,
            updated_at
           FROM servers WHERE id = ?
        "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let server_instance: Server = Server::from(row);
        Ok(Some(server_instance))
    }
    async fn create(&self, new_server: NewServer) -> Result<Server> {
        let insert = NewServerRow::from(new_server);
        let result = sqlx::query(
            r#"
        INSERT INTO servers (
            name, 
            slug,
            is_active,
            path,
            jar_path,
            j_max_mem,
            j_min_mem,
            mc_type,
            mc_version,
            created_at,
            updated_at
        ) VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(insert.name)
        .bind(insert.slug)
        .bind(insert.is_active)
        .bind(insert.path)
        .bind(insert.jar_path)
        .bind(insert.j_max_mem)
        .bind(insert.j_min_mem)
        .bind(insert.mc_type)
        .bind(insert.mc_version)
        .bind(insert.created_at)
        .bind(insert.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!(
                target = "db",
                error = ?e,
                "Failed to insert new server into DB"
            );
            e
        })?;

        let id = result.last_insert_rowid();

        let newserver: Server = self
            .get_by_id(id)
            .await
            .map_err(|e| {
                error!(
                    target = "db",
                    %id,
                    error = ?e,
                    "Failed to fetch server by id after insert"
                );
                e
            })?
            .ok_or_else(|| {
                let err = anyhow::anyhow!("Server failed to add to DB: no row found for id {}", id);
                error!(
                    target = "db",
                    %id,
                    error = %err,
                    "Inserted server not found when reloading"
                );
                err
            })?;

        Ok(newserver)
    }
}
