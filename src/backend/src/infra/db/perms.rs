use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::user_prems::{UserPermissions, UserPermissionsRow},
    prelude::*,
};

pub async fn create(
    pool: &PgPool,
    uuid: Uuid,
    new_perms: UserPermissions,
) -> Result<UserPermissions> {
    debug!(user_uuid = %uuid, "insert user permissions started");
    let insert = UserPermissionsRow::from(new_perms);
    let perms = sqlx::query_as::<_, UserPermissionsRow>(
        r#"
    INSERT INTO user_permissions (uuid, root, permissions)
    VALUES ($1, $2, $3)
    RETURNING root, permissions
    "#,
    )
    .bind(uuid)
    .bind(insert.root)
    .bind(insert.permissions)
    .fetch_one(pool)
    .await?;

    debug!(user_uuid = %uuid, "insert user permissions completed");
    Ok(UserPermissions::from(perms))
}

pub async fn get_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<UserPermissions>> {
    debug!(user_uuid = %uuid, "fetch user permissions by uuid started");
    let perms = sqlx::query_as::<_, UserPermissionsRow>(
        r#"
    SELECT uuid, root, permissions
    FROM user_permissions
    WHERE uuid = $1
    "#,
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?;

    debug!(user_uuid = %uuid, "fetch user permissions by uuid completed");
    match perms {
        Some(val) => return Ok(Some(UserPermissions::from(val))),
        None => return Ok(None),
    }
}

pub async fn exists_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<bool> {
    debug!(user_uuid = %uuid, "check user permissions existence started");
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
    SELECT EXISTS(
        SELECT 1
        FROM user_permissions
        WHERE uuid = $1
    )
    "#,
    )
    .bind(uuid)
    .fetch_one(pool)
    .await?;

    debug!(user_uuid = %uuid, "check user permissions existence completed");
    Ok(exists)
}
