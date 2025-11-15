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
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: i64,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_login_at: Option<String>,
}

impl From<NewUser> for UserRow {
    fn from(value: NewUser) -> Self {
        UserRow {
            id: -1,
            username: value.username,
            password_hash: value.password_hash,
            role: value.role.to_string(),
            is_active: 1,
            email: value.email,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            last_login_at: None,
        }
    }
}

impl From<User> for UserRow {
    fn from(value: User) -> Self {
        UserRow {
            id: value.id,
            username: value.username,
            password_hash: value.password_hash,
            role: value.role.to_string(),
            is_active: value.is_active as i64,
            email: value.email,
            created_at: value.created_at.to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            last_login_at: value.last_login_at.map(|dt| dt.to_rfc3339()),
        }
    }
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

        let user: User = User::from(row);

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

        let user: User = User::from(row);

        Ok(Some(user))
    }
    async fn create(&self, user: NewUser) -> Result<User> {
        let insert: UserRow = UserRow::from(user);

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
        .bind(insert.username)
        .bind(insert.password_hash)
        .bind(insert.role)
        .bind(insert.is_active)
        .bind(insert.email)
        .bind(insert.created_at)
        .bind(insert.updated_at)
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
        let users_rows = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT *
            FROM users
            ORDER BY id
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let users: Vec<User> = users_rows.into_iter().map(User::from).collect();

        Ok(users)
    }
    async fn update(&self, user: User) -> Result<User> {
        Ok(user)
    }
    async fn timestamp_login(&self, id: i64) -> Result<()> {
        Ok(())
    }
    async fn delete(&self, id: i64) -> Result<()> {
        let _ = sqlx::query(
            r#"
            DELETE FROM users WHERE id = ?
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
