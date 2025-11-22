use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::validation::{slugify, validate_name};

#[derive(Debug, Clone)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub path: String,
    pub jar_path: String,
    pub j_max_mem: String,
    pub j_min_mem: String,
    pub mc_type: String,
    pub mc_version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SafeServer {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub mc_type: String,
    pub mc_version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewServer {
    pub name: String,
    pub path: String,
    pub jar_path: String,
    pub j_max_mem: String,
    pub j_min_mem: String,
    pub mc_type: String,
    pub mc_version: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct ServerRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: i64,
    pub path: String,
    pub jar_path: String,
    pub j_max_mem: String,
    pub j_min_mem: String,
    pub mc_type: String,
    pub mc_version: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct NewServerRow {
    pub name: String,
    pub slug: String,
    pub is_active: i64,
    pub path: String,
    pub jar_path: String,
    pub j_max_mem: String,
    pub j_min_mem: String,
    pub mc_type: String,
    pub mc_version: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Server> for SafeServer {
    fn from(value: Server) -> Self {
        SafeServer {
            id: value.id,
            name: value.name,
            slug: value.slug,
            is_active: value.is_active,
            mc_type: value.mc_type,
            mc_version: value.mc_version,
            created_at: value.created_at,
            updated_at: value.created_at,
        }
    }
}

impl From<ServerRow> for Server {
    fn from(value: ServerRow) -> Self {
        Server {
            id: value.id,
            name: value.name,
            slug: value.slug,
            is_active: value.is_active != 0,
            path: value.path,
            jar_path: value.jar_path,
            j_max_mem: value.j_max_mem,
            j_min_mem: value.j_min_mem,
            mc_type: value.mc_type,
            mc_version: value.mc_version,
            created_at: DateTime::parse_from_rfc3339(&value.created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&value.updated_at)
                .unwrap()
                .with_timezone(&Utc),
        }
    }
}

impl From<NewServer> for NewServerRow {
    fn from(value: NewServer) -> Self {
        NewServerRow {
            name: validate_name(&value.name),
            slug: slugify(&value.name),
            is_active: 1,
            path: value.path,
            jar_path: value.jar_path,
            j_max_mem: value.j_max_mem,
            j_min_mem: value.j_min_mem,
            mc_type: value.mc_type,
            mc_version: value.mc_version,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }
}
