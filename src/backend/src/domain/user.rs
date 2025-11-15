use std::str::FromStr;

use crate::infrastructure::sqlite_user_repo::UserRow;
use chrono::{DateTime, Utc};
#[derive(Clone, Debug)]
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

#[derive(Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub email: Option<String>,
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
            last_login_at: Some(
                DateTime::parse_from_rfc3339(&row.last_login_at.unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        }
    }
}
