use crate::{
    auth::{gen_jwt, verify_password},
    domain::{api::LoginData, user::InternalUser, user_prems::UserPermissions},
    infra::db,
    prelude::*,
};
use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use anyhow::Context;

use crate::{
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};

pub async fn login(state: Arc<AppState>, login_data: LoginData) -> Result<String, StatusCode> {
    debug!(username = login_data.username.as_str(), "login started");

    let user = db::user::get_by_username(&state.db_pool, &login_data.username)
        .await
        .map_err(|e| {
            error!(error = %e, username = login_data.username.as_str(), "fetch user during login failed");
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;
    let user = match user {
        Some(value) => value,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let verify = verify_password(&login_data.password, &user.password_hash).map_err(|e| {
        error!(error = %e, username = login_data.username.as_str(), "verify password hash failed");
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;

    if !verify {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = gen_jwt(user.username.clone()).map_err(|e| {
        error!(error = %e, username = login_data.username.as_str(), "generate jwt failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(token)
}

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

    let mut created_user = db::user::create(&state.db_pool, internal.clone())
        .await
        .map_err(|e| {
            error!(error = %e, "create user failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let perms = db::perms::create(
        &state.db_pool,
        created_user.uuid.clone(),
        internal.permissions,
    )
    .await
    .map_err(|e| {
        error!(error = %e, "create user permissions entry failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    created_user.attach_permissions(perms);

    let created_user = created_user;

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

    let mut user = match db::user::get_safe_by_uuid(&state.db_pool, uuid)
        .await
        .map_err(|e| {
            error!(error = %e, user_uuid = %uuid, "fetch user failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })? {
        Some(u) => u,
        None => return Ok(None),
    };

    let perms = db::perms::get_by_uuid(&state.db_pool, uuid)
        .await
        .map_err(|e| {
            error!(error = %e, user_uuid = %uuid, "fetch permissions failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match perms {
        Some(perms) => {
            user.attach_permissions(perms);
        }
        None => {
            error!(user_uuid = %uuid, "permissions missing for existing user");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Some(user))
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

    let perms = db::perms::get_by_uuid(&state.db_pool, user.uuid)
        .await
        .context("failed to fetch user permissions")?;

    match perms {
        Some(perms) => {
            user.attach_permissions(perms);
        }
        None => {}
    }

    Ok(Some(user))
}

pub async fn get_safe_by_username(
    state: Arc<AppState>,
    username: &str,
) -> Result<Option<User>, StatusCode> {
    debug!(%username, "fetch user by username started");

    let mut user = match db::user::get_safe_by_username(&state.db_pool, username)
        .await
        .map_err(|e| {
            error!(error = %e, %username, "fetch user by username failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })? {
        Some(u) => u,
        None => return Ok(None),
    };

    let perms = db::perms::get_by_uuid(&state.db_pool, user.uuid)
        .await
        .map_err(|e| {
            error!(error = %e, user_uuid = %user.uuid, "fetch permissions failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match perms {
        Some(perms) => {
            user.attach_permissions(perms);
        }
        None => {
            error!(user_uuid = %user.uuid, "permissions missing for existing user");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Some(user))
}

pub async fn get_by_username(
    state: Arc<AppState>,
    username: &str,
) -> anyhow::Result<Option<InternalUser>> {
    debug!(%username, "fetch internal user by username started");

    let mut user = match db::user::get_by_username(&state.db_pool, username)
        .await
        .context("failed to fetch user by username")?
    {
        Some(u) => u,
        None => return Ok(None),
    };

    let perms = db::perms::get_by_uuid(&state.db_pool, user.uuid)
        .await
        .context("failed to fetch user permissions")?;

    match perms {
        Some(perms) => {
            user.attach_permissions(perms);
        }
        None => {}
    }

    Ok(Some(user))
}
