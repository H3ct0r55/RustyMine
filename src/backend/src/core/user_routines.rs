use crate::{
    domain::{user::InternalUser, user_prems::UserPermissions},
    infra::db,
    prelude::*,
};
use std::sync::Arc;

use axum::http::StatusCode;
use uuid::Uuid;
use validator::Validate;

use anyhow::Context;

use crate::{
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};

pub async fn create(state: Arc<AppState>, new_user: NewUser) -> Result<User, StatusCode> {
    debug!("create user started");

    new_user.validate().map_err(|e| {
        error!(error = %e, "user validation failed");
        StatusCode::BAD_REQUEST
    })?;

    let internal = InternalNewUser::try_from(new_user).map_err(|e| {
        error!(error = %e, "convert to internal user failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let created_user = db::user::create(&state.db_pool, internal)
        .await
        .map_err(|e| {
            error!(error = %e, "create user failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(user_uuid = %created_user.uuid, "user created");
    Ok(User::from(created_user))
}

pub async fn get_all(state: Arc<AppState>) -> Result<Vec<User>, StatusCode> {
    debug!("fetch all users started");
    let users = db::user::get_safe_all(&state.db_pool).await.map_err(|e| {
        error!(error = %e, "fetch all users failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(users)
}

pub async fn get_safe_by_uuid(
    state: Arc<AppState>,
    uuid: Uuid,
) -> Result<Option<User>, StatusCode> {
    debug!(user_uuid = %uuid, "fetch user by uuid started");
    let user = db::user::get_safe_by_uuid(&state.db_pool, uuid.clone())
        .await
        .map_err(|e| {
            error!(error = %e, user_uuid = %uuid, "fetch user failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(user)
}

pub async fn get_by_uuid(state: Arc<AppState>, uuid: Uuid) -> anyhow::Result<Option<InternalUser>> {
    debug!(user_uuid = %uuid, "fetch internal user started");
    let mut user = match db::user::get_by_uuid(&state.db_pool, uuid)
        .await
        .context("failed to fetch user by uuid")?
    {
        Some(u) => u,
        None => return Ok(None),
    };

    let perms_exist = db::perms::exists_by_uuid(&state.db_pool, user.uuid)
        .await
        .context("failed to check if user permissions exist")?;

    if perms_exist {
        if let Some(perms) = db::perms::get_by_uuid(&state.db_pool, user.uuid)
            .await
            .context("failed to fetch user permissions")?
        {
            user.attach_permissions(perms);
        }
    }

    let user = user;

    Ok(Some(user))
}

pub async fn set_perms(
    state: Arc<AppState>,
    user_perms: UserPermissions,
) -> Result<(), StatusCode> {
    debug!(user_uuid = %user_perms.uuid, "assign permissions started");
    let exists = db::user::exists_by_uuid(&state.db_pool, user_perms.uuid)
        .await
        .map_err(|e| {
            error!(error = %e, user_uuid = %user_perms.uuid, "verify user exists failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !exists {
        warn!(user_uuid = %user_perms.uuid, "assign permissions skipped for missing user");
        return Err(StatusCode::BAD_REQUEST);
    }

    db::perms::create(&state.db_pool, user_perms)
        .await
        .map_err(|e| {
            error!(error = %e, "create user permissions entry failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("user permissions assigned");
    Ok(())
}
