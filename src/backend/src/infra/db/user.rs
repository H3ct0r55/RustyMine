use crate::{
    domain::user::{InternalNewUser, InternalUser, User},
    prelude::*,
};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(pool: &PgPool, new_user: InternalNewUser) -> Result<InternalUser> {
    let user = sqlx::query_as::<_, InternalUser>(
        r#"
        INSERT INTO users (uuid, username, email, password_hash, first_name, last_name)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING uuid, username, email, password_hash, first_name, last_name
        "#,
    )
    .bind(new_user.uuid)
    .bind(&new_user.username)
    .bind(&new_user.email)
    .bind(&new_user.password_hash)
    .bind(&new_user.first_name)
    .bind(&new_user.last_name)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<InternalUser>> {
    let user = sqlx::query_as::<_, InternalUser>(
        r#"
    SELECT uuid, username, email, password_hash, first_name, last_name FROM users WHERE uuid = $1
    "#,
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_safe_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        r#"
    SELECT uuid, username, email, first_name, last_name FROM users WHERE uuid = $1
    "#,
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<InternalUser>> {
    let users = sqlx::query_as::<_, InternalUser>(
        r#"
        SELECT uuid, username, email, password_hash, first_name, last_name
        FROM users
        ORDER BY username ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn get_safe_all(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT uuid, username, email, first_name, last_name
        FROM users
        ORDER BY username ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn exists_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM users
            WHERE uuid = $1
        )
        "#,
    )
    .bind(uuid)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}
