use std::{fmt, str::FromStr};

use crate::infrastructure::sqlite_user_repo::UserRow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            _ => Err(format!("Unknonw role: {}", s)),
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

impl<'de> Deserialize<'de> for UserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        UserRole::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeUser {
    pub id: i64,
    pub username: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub email: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub email: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    #[serde(default)]
    pub password_hash: String,
    pub role: UserRole,
    #[serde(default)]
    pub email: Option<String>,
}

impl From<User> for SafeUser {
    fn from(value: User) -> Self {
        SafeUser {
            id: value.id,
            username: value.username,
            role: value.role,
            created_at: value.created_at,
            updated_at: value.updated_at,
            is_active: value.is_active,
            email: value.email,
            last_login_at: value.last_login_at,
        }
    }
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            username: row.username,
            password_hash: row.password_hash,
            role: UserRole::from_str(&row.role).unwrap(),
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap()
                .with_timezone(&Utc),
            is_active: row.is_active != 0,
            email: row.email,
            last_login_at: row
                .last_login_at
                .as_deref()
                .map(|s| DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)),
        }
    }
}
