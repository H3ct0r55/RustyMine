use chrono::{DateTime, Utc};
#[derive(Clone, Debug)]
pub enum UserRole {
    Admin,
}

//DB SCHEMA
//id PRIMARY KEY INTEGER
//username TEXT NOT NULL
//password_hash TEXT NOT NULL
//role TEXT NOT NULL
//created_at DATETIME NOT NULL
//updated_at DATETIME NOT NULL
//is_active INTEGER NOT NULL
//email TEXT
//last_login_at DATETIME NOT NULL

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
