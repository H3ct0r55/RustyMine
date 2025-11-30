pub mod middleware;
pub mod user_routes;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::{Layer, ServiceBuilder};

use crate::state::AppState;

pub async fn init_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/ping",
            get(ping).layer(
                ServiceBuilder::new()
                    .layer(middleware::cors())
                    .layer(axum::middleware::from_fn(middleware::auth)),
            ),
        )
        .route(
            "/api/users",
            post(user_routes::create)
                .layer(ServiceBuilder::new().layer(middleware::cors()))
                .with_state(app_state.clone())
                .get(user_routes::get_all)
                .layer(ServiceBuilder::new().layer(middleware::cors()))
                .with_state(app_state.clone()),
        )
        .route("/api/users/{uuid}", get(user_routes::get_uuid))
        .layer(ServiceBuilder::new().layer(middleware::cors()))
        .with_state(app_state.clone())
}

async fn ping() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "response": "pong"})))
}
