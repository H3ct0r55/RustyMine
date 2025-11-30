use crate::prelude::*;
use std::sync::Arc;

use crate::{
    core,
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};
use anyhow::Result;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let user = core::user_routines::create(state, new_user).await?;
    Ok(Json(user))
}

pub async fn get_all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = core::user_routines::get_all(state).await?;
    Ok(Json(users))
}

pub async fn get_uuid(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Option<User>>, StatusCode> {
    let user = core::user_routines::get_safe_by_uuid(state, uuid).await?;
    Ok(Json(user))
}
