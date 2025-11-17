use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SafeUser {
    pub id: i64,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub is_active: i64,
    pub created_at: String,
    pub updated_at: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct NewUserRow {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub is_active: i64,
    pub created_at: String,
    pub updated_at: String,
    pub password_hash: String,
}

impl From<User> for SafeUser {
    fn from(value: User) -> Self {
        SafeUser {
            id: value.id,
            username: value.username,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            role: value.role,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        User {
            id: value.id,
            username: value.username,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            role: value.role,
            is_active: value.is_active != 0,
            created_at: DateTime::parse_from_rfc3339(&value.created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&value.updated_at)
                .unwrap()
                .with_timezone(&Utc),
            password_hash: value.password_hash,
        }
    }
}

impl From<NewUser> for NewUserRow {
    fn from(value: NewUser) -> Self {
        NewUserRow {
            username: value.username,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            role: value.role,
            is_active: 1_i64,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            password_hash: value.password_hash,
        }
    }
}
