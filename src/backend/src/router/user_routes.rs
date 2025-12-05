use crate::{domain::api::LoginData, prelude::*};
use std::sync::Arc;

use crate::{
    core,
    domain::user::{NewUser, User},
    state::AppState,
};
use anyhow::Result;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use uuid::Uuid;

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    debug!("create user route started");
    let user = core::user_routines::create(state, new_user).await?;
    info!("create user route completed");
    Ok(Json(user))
}

pub async fn get_all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, StatusCode> {
    debug!("list users route started");
    let users = core::user_routines::get_all(state).await?;
    debug!(user_count = users.len(), "list users route completed");
    Ok(Json(users))
}

pub async fn get_uuid(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Option<User>>, StatusCode> {
    debug!(user_uuid = %uuid, "get user by uuid route started");
    let user = core::user_routines::get_safe_by_uuid(state, uuid).await?;
    debug!("get user by uuid route completed");
    Ok(Json(user))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(login_data): Json<LoginData>,
) -> Result<(CookieJar, Json<User>), StatusCode> {
    let (jwt, user) = core::user_routines::login(state, login_data).await?;

    let cookie = Cookie::build(("auth_token", jwt))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::None)
        .path("/")
        .build();

    let jar = jar.add(cookie);

    Ok((jar, Json(user)))
}

pub async fn logout(jar: CookieJar) -> Result<CookieJar, StatusCode> {
    let cookie = Cookie::build(("auth_token", ""))
        .path("/")
        .http_only(true)
        .build();
    let jar = jar.remove(cookie);

    Ok(jar)
}
