use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{SqlitePool, prelude::FromRow};

use crate::domain::{
    repository::ServerRepository,
    server::{NewServerInstance, ServerInstance},
};

pub struct SqliteServerRepository {
    pool: SqlitePool,
}

#[derive(Debug, FromRow)]
pub struct ServerRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub mc_version: String,
    pub port: i64,
    pub rcon_enabled: i64,
    pub rcon_port: Option<i64>,
    pub j_max_memory_mb: i64,
    pub j_min_memory_mb: i64,
    pub created_at: String,
    pub updated_at: String,
    pub last_started_at: Option<String>,
}

impl From<NewServerInstance> for ServerRow {
    fn from(value: NewServerInstance) -> Self {
        ServerRow {
            id: -1,
            name: value.name.clone(),
            slug: value.name.to_lowercase().replace(" ", "-"),
            mc_version: value.mc_version.to_string(),
            port: value.port as i64,
            rcon_enabled: value.rcon_enabled as i64,
            rcon_port: value.rcon_port.map(|p| p as i64),
            j_max_memory_mb: value.j_max_memory_mb as i64,
            j_min_memory_mb: value.j_min_memory_mb as i64,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            last_started_at: None,
        }
    }
}

impl From<ServerInstance> for ServerRow {
    fn from(value: ServerInstance) -> Self {
        ServerRow {
            id: value.id,
            name: value.name.clone(),
            slug: value
                .name
                .clone()
                .to_string()
                .to_lowercase()
                .replace(" ", "-")
                .to_string(),
            mc_version: value.mc_version.to_string(),
            port: value.port as i64,
            rcon_enabled: value.rcon_enabled as i64,
            rcon_port: value.rcon_port.map(|p| p as i64),
            j_max_memory_mb: value.j_max_memory_mb as i64,
            j_min_memory_mb: value.j_min_memory_mb as i64,
            created_at: value.created_at.to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            last_started_at: value.last_started_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

impl SqliteServerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServerRepository for SqliteServerRepository {
    async fn get_by_id(&self, id: i64) -> Result<Option<ServerInstance>> {
        let row: Option<ServerRow> = sqlx::query_as::<_, ServerRow>(r#"
           SELECT id, name, slug, mc_version, port, rcon_enabled, rcon_port, j_max_memory_mb, j_min_memory_mb, created_at, updated_at, last_started_at
           FROM servers WHERE id = ?
        "#).bind(id).fetch_optional(&self.pool).await?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let server_instance: ServerInstance = ServerInstance::from(row);
        Ok(Some(server_instance))
    }
    async fn get_by_name(&self, name: &str) -> Result<Option<ServerInstance>> {
        let row: Option<ServerRow> = sqlx::query_as::<_, ServerRow>(r#"
           SELECT id, name, slug, mc_version, port, rcon_enabled, rcon_port, j_max_memory_mb, j_min_memory_mb, created_at, updated_at, last_started_at
           FROM servers WHERE name = ?
        "#).bind(name).fetch_optional(&self.pool).await?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let server_instance: ServerInstance = ServerInstance::from(row);
        Ok(Some(server_instance))
    }
    async fn get_by_slug(&self, slug: &str) -> Result<Option<ServerInstance>> {
        let row: Option<ServerRow> = sqlx::query_as::<_, ServerRow>(r#"
           SELECT id, name, slug, mc_version, port, rcon_enabled, rcon_port, j_max_memory_mb, j_min_memory_mb, created_at, updated_at, last_started_at
           FROM servers WHERE slug = ?
        "#).bind(slug).fetch_optional(&self.pool).await?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };
        let server_instance: ServerInstance = ServerInstance::from(row);
        Ok(Some(server_instance))
    }
    async fn create(&self, server_instance: NewServerInstance) -> Result<ServerInstance> {
        let insert = ServerRow::from(server_instance);
        let result = sqlx::query(
            r#"
            INSERT INTO servers (
               name, 
               slug,
               mc_version,
               port,
               rcon_enabled,
               rcon_port,
               j_max_memory_mb,
               j_min_memory_mb,
               created_at,
               updated_at,
               last_started_at
            ) VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL)
        "#,
        )
        .bind(insert.name)
        .bind(insert.slug)
        .bind(insert.mc_version)
        .bind(insert.port)
        .bind(insert.rcon_enabled)
        .bind(insert.rcon_port)
        .bind(insert.j_max_memory_mb)
        .bind(insert.j_min_memory_mb)
        .bind(insert.created_at)
        .bind(insert.updated_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_rowid();
        let newserver: ServerInstance = self
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Server failed to add to DB"))?;
        Ok(newserver)
    }
    async fn list_all(&self) -> Result<Vec<ServerInstance>> {
        let server_rows = sqlx::query_as::<_, ServerRow>(
            r#"
            SELECT *
            FROM servers
            ORDER BY id
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let servers: Vec<ServerInstance> =
            server_rows.into_iter().map(ServerInstance::from).collect();
        Ok(servers)
    }
    async fn update(&self, server_instance: ServerInstance) -> Result<ServerInstance> {
        let update = ServerRow::from(server_instance);
        let _ = sqlx::query(r#"
            UPDATE servers
            SET name = ?, slug = ?, mc_version = ?, port = ?, rcon_enabled = ?, rcon_port = ?, j_max_memory_mb = ?, j_min_memory_mb = ?, updated_at = ?, last_started_at = ?
            WHERE id = ?
        "#)
        .bind(update.name)
        .bind(update.slug)
        .bind(update.mc_version)
        .bind(update.port)
        .bind(update.rcon_enabled)
        .bind(update.rcon_port)
        .bind(update.j_max_memory_mb)
        .bind(update.j_min_memory_mb)
        .bind(update.updated_at)
        .bind(update.last_started_at)
        .bind(update.id)
        .execute(&self.pool)
        .await?;
        let updatedserver: ServerInstance = self
            .get_by_id(update.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to update servers DB at id {}", update.id))?;
        Ok(updatedserver)
    }
    async fn timestamp_start(&self, id: i64) -> Result<()> {
        Ok(())
    }
    async fn delete(&self, id: i64) -> Result<()> {
        let _ = sqlx::query(
            r#"
            DELETE FROM servers WHERE id = ?
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
