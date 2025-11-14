use crate::domain::{
    repository::UserRepository,
    user::{NewUser, User, UserRole},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

#[derive(Debug, FromRow)]
struct UserRow {
    id: i64,
    username: String,
    password_hash: String,
    role: String,
    is_active: i64,
    email: Option<String>,
    created_at: String,
    updated_at: String,
    last_login_at: Option<String>,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

//TODO: remove duplication between get_by_id and get_by_username and migrate matching to single
//helper

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn get_by_id(&self, id: i64) -> Result<Option<User>> {
        let row: Option<UserRow> = sqlx::query_as::<_, UserRow>(r#"
            SELECT id, username, password_hash, role, is_active, email, created_at, updated_at, last_login_at
            FROM users
            WHERE id = ?
        "#).bind(id).fetch_optional(&self.pool).await?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let role = match row.role.as_str() {
            "admin" => UserRole::Admin,
            other => anyhow::bail!("unknown user role in DB: {}", other),
        };

        let created_at = DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc);
        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)?.with_timezone(&Utc);

        let last_login_at = if let Some(s) = row.last_login_at {
            Some(DateTime::parse_from_rfc3339(&s)?.with_timezone(&Utc))
        } else {
            None
        };

        let user = User {
            id: row.id,
            username: row.username,
            password_hash: row.password_hash,
            role,
            created_at,
            updated_at,
            is_active: row.is_active != 0,
            email: row.email,
            last_login_at,
        };

        Ok(Some(user))
    }
    async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        let row: Option<UserRow> = sqlx::query_as::<_, UserRow>(r#"
            SELECT id, username, password_hash, role, is_active, email, created_at, updated_at, last_login_at
            FROM users
            WHERE username = ?
        "#).bind(username).fetch_optional(&self.pool).await?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let role = match row.role.as_str() {
            "admin" => UserRole::Admin,
            other => anyhow::bail!("unknown user role in DB: {}", other),
        };

        let created_at = DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc);
        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)?.with_timezone(&Utc);

        let last_login_at = if let Some(s) = row.last_login_at {
            Some(DateTime::parse_from_rfc3339(&s)?.with_timezone(&Utc))
        } else {
            None
        };

        let user = User {
            id: row.id,
            username: row.username,
            password_hash: row.password_hash,
            role,
            created_at,
            updated_at,
            is_active: row.is_active != 0,
            email: row.email,
            last_login_at,
        };

        Ok(Some(user))
    }
    async fn create(&self, user: NewUser) -> Result<User> {
        let role_str = match user.role {
            UserRole::Admin => "admin",
        };

        let now = Utc::now();
        let created_at_str = now.to_rfc3339();
        let updated_at_str = created_at_str.clone();

        let result = sqlx::query(
            r#"
            INSERT INTO users (
                username,
                password_hash,
                role,
                is_active,
                email,
                created_at,
                updated_at,
                last_login_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, NULL) 
        "#,
        )
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(role_str)
        .bind(1_i64)
        .bind(&user.email)
        .bind(&created_at_str)
        .bind(&updated_at_str)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid();
        let newuser: User = self
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User failed to add to DB"))?;
        Ok(newuser)
    }
    async fn list_all(&self) -> Result<Vec<User>> {
        Ok(vec![])
    }
    async fn update(&self, user: User) -> Result<User> {
        Ok(user)
    }
    async fn timestamp_login(&self, id: i64) -> Result<()> {
        Ok(())
    }
    async fn delete(&self, id: i64) -> Result<()> {
        Ok(())
    }
}
