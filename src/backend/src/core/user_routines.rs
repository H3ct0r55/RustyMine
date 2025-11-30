use crate::{
    domain::{
        user::{self, InternalUser},
        user_prems::UserPermissions,
    },
    infra::db,
    prelude::*,
};
use std::sync::Arc;

use axum::{Json, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use anyhow::{Context, anyhow};

use crate::{
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};

pub async fn create(state: Arc<AppState>, new_user: NewUser) -> Result<User, StatusCode> {
    new_user.validate().map_err(|e| {
        error!("User validation failed: {e}");
        StatusCode::BAD_REQUEST
    })?;

    let internal = InternalNewUser::try_from(new_user).map_err(|e| {
        error!("Conversion to InternalUser failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let created_user = db::user::create(&state.db_pool, internal)
        .await
        .map_err(|e| {
            error!("Failed to create new user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(User::from(created_user))
}

pub async fn get_all(state: Arc<AppState>) -> Result<Vec<User>, StatusCode> {
    let users = db::user::get_safe_all(&state.db_pool).await.map_err(|e| {
        error!("Failed to fetch all users: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(users)
}

pub async fn get_safe_by_uuid(
    state: Arc<AppState>,
    uuid: Uuid,
) -> Result<Option<User>, StatusCode> {
    let user = db::user::get_safe_by_uuid(&state.db_pool, uuid.clone())
        .await
        .map_err(|e| {
            error!("Failed to fetch user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(user)
}

pub async fn get_by_uuid(state: Arc<AppState>, uuid: Uuid) -> anyhow::Result<Option<InternalUser>> {
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
    let exists = db::user::exists_by_uuid(&state.db_pool, user_perms.uuid)
        .await
        .map_err(|e| {
            error!("Failed to verify user exists: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    db::perms::create(&state.db_pool, user_perms)
        .await
        .map_err(|e| {
            error!("Failed to create user permissions entry: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(())
}
