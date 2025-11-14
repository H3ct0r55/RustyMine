use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub enum McVersion {
    V1_21_1,
    V1_21_10,
}

//DB SCHEMA
//id PRIMARY KEY INTEGER
//name TEXT NOT NULL
//mc_version TEXT NOT NULL
//port INTEGER NOT NULL
//rcon_enabled INTEGER NOT NULL
//rcon_port INTEGER
//created_at DATETIME NOT NULL
//updated_at DATETIME NOT NULL
//last_started_at DATETIME
//j_max_memory_mb INTEGER NOT NULL
//j_min_memory_mb INTEGER NOT NULL

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
