use std::sync::Arc;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde_json::{Value, json};
use tracing::{error, info, warn};

use crate::{
    domain::server::{NewServer, SafeServer},
    infrastructure::state::AppState,
};

pub fn build_api_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1", api_v1_routes())
        .with_state(state)
}

fn api_v1_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/server/add", post(server_add))
        .route("/server/{id}", get(server_get_by_id))
        .route("/server/{id}/start", post(server_start))
        .route("/server/{id}/stop", post(server_stop))
}

async fn server_add(
    State(state): State<Arc<AppState>>,
    Json(input): Json<NewServer>,
) -> Result<Json<Value>, StatusCode> {
    info!(target: "api", name = %input.name, "Creating server via /api/v1/server/add");

    let server = state.server_repo.create(input).await.map_err(|e| {
        error!(
            target = "api",
            error = ?e,
            "Failed to create server via /api/v1/server/add"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let safe_server = SafeServer::from(server);

    Ok(Json(json!({
        "ok": true,
        "server": safe_server
    })))
}

// GET /api/v1/server/:id
async fn server_get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    info!(target = "api", id, "Fetching server via /api/v1/server/:id");

    // Assuming: get_by_id(id) -> Result<Option<Server>, E>
    let maybe_server = state.server_repo.get_by_id(id).await.map_err(|e| {
        error!(
            target = "api",
            error = ?e,
            id,
            "Failed to fetch server via get_by_id"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let server = match maybe_server {
        Some(s) => s,
        None => {
            // 404 Not Found if no server with this id
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let safe_server = SafeServer::from(server);

    Ok(Json(json!({
        "ok": true,
        "server": safe_server
    })))
}

async fn server_start(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        target = "api",
        id, "Starting server via /api/v1/server/:id/start"
    );

    let maybe_server = state.server_repo.get_by_id(id).await.map_err(|e| {
        error!(
            target = "api",
            error = ?e,
            id,
            "Failed to fetch server via get_by_id"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let server = match maybe_server {
        Some(s) => s,
        None => {
            // 404 Not Found if no server with this id
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let result = state.supervisor.start_server(server).await;

    match result {
        Ok(()) => Ok(Json(json!({"ok": true}))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn server_stop(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        target = "api",
        id, "Stoppeing server via /api/v1/server/:id/stop"
    );

    match state.supervisor.stop_server(id).await {
        Ok(()) => Ok(Json(json!({"ok": true}))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
