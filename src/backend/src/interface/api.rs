use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde_json::{Value, json};

use crate::{domain::server::NewServer, infrastructure::state::AppState};

pub fn build_api_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1", api_v1_routes())
        .with_state(state)
}

fn api_v1_routes() -> Router<Arc<AppState>> {
    Router::new().route("/server/add", post(server_add))
}

async fn server_add(
    State(state): State<Arc<AppState>>,
    Json(input): Json<NewServer>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({"ok": true})))
}
