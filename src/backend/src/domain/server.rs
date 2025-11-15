use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::infrastructure::sqlite_server_repo::ServerRow;

#[derive(Clone, Debug)]
pub enum McVersion {
    V1_21_1,
    V1_21_10,
}

impl FromStr for McVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1.21.1" => Ok(McVersion::V1_21_1),
            "1.12.10" => Ok(McVersion::V1_21_10),
            _ => Err(format!("Unknown minecraft version: {:?}", s)),
        }
    }
}

impl fmt::Display for McVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            McVersion::V1_21_1 => write!(f, "1.21.1"),
            McVersion::V1_21_10 => write!(f, "1.21.10"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerInstance {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub mc_version: McVersion,
    pub port: u16,
    pub rcon_enabled: bool,
    pub rcon_port: Option<u16>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_started_at: Option<DateTime<Utc>>,
    pub j_max_memory_mb: u32,
    pub j_min_memory_mb: u32,
}

#[derive(Clone, Debug)]
pub struct NewServerInstance {
    pub name: String,
    pub mc_version: McVersion,
    pub port: u16,
    pub rcon_enabled: bool,
    pub rcon_port: Option<u16>,
    pub j_max_memory_mb: u32,
    pub j_min_memory_mb: u32,
}

impl From<ServerRow> for ServerInstance {
    fn from(row: ServerRow) -> Self {
        ServerInstance {
            id: row.id,
            name: row.name,
            slug: row.slug,
            mc_version: McVersion::from_str(&row.mc_version).unwrap(),
            port: row.port as u16,
            rcon_enabled: row.rcon_enabled != 0,
            rcon_port: row.rcon_port.map(|p| p as u16),
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap()
                .with_timezone(&Utc),
            last_started_at: row
                .last_started_at
                .as_deref()
                .map(|s| DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)),
            j_max_memory_mb: row.j_max_memory_mb as u32,
            j_min_memory_mb: row.j_min_memory_mb as u32,
        }
    }
}
