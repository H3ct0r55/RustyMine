use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{user::User, user_prems::UserPermissions};

pub async fn create(pool: &PgPool, new_perms: UserPermissions) -> Result<UserPermissions> {
    let perms = sqlx::query_as::<_, UserPermissions>(
        r#"
    INSERT INTO user_permissions (uuid, root, manage_users, login)
    VALUES ($1, $2, $3, $4)
    RETURNING uuid, root, manage_users, login
    "#,
    )
    .bind(new_perms.uuid)
    .bind(new_perms.root)
    .bind(new_perms.manage_users)
    .bind(new_perms.login)
    .fetch_one(pool)
    .await?;

    Ok(perms)
}

pub async fn get_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<UserPermissions>> {
    let perms = sqlx::query_as::<_, UserPermissions>(
        r#"
    SELECT uuid, root, manage_users, login
    FROM user_permissions
    WHERE uuid = $1
    "#,
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?;

    Ok(perms)
}

pub async fn exists_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
    SELECT EXISTS(
        SELECT 1
        FROM user_permissions
        WHERE uuis = $1
    )
    "#,
    )
    .bind(uuid)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}
