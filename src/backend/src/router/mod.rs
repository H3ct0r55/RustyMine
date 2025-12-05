pub mod middleware;
pub mod user_routes;

use axum::{
    Json, Router,
    http::StatusCode,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::{Layer, ServiceBuilder};

use crate::prelude::*;
use crate::state::AppState;

macro_rules! middleware {
    (cors) => {
        crate::router::middleware::cors()
    };
    (cors_auth, $state:expr) => {
        (
            crate::router::middleware::cors(),
            axum::middleware::from_fn_with_state($state, crate::router::middleware::auth),
        )
    };
    (cors_auth_perms, $state:expr) => {
        (
            crate::router::middleware::cors(),
            axum::middleware::from_fn_with_state($state, crate::router::middleware::auth),
            axum::middleware::from_fn_with_state($state, crate::router::middleware::permissions),
        )
    };
}

pub async fn init_router(app_state: Arc<AppState>) -> Router {
    info!("router initialization started");

    let router = Router::new()
        .route("/api/ping", get(ping).layer(middleware!(cors)))
        .route(
            "/api/users",
            post(user_routes::create)
                .layer(middleware!(cors_auth_perms, app_state.clone()))
                .with_state(app_state.clone())
                .get(user_routes::get_all)
                .layer(middleware!(cors_auth_perms, app_state.clone()))
                .with_state(app_state.clone()),
        )
        .route(
            "/api/users/{uuid}",
            get(user_routes::get_uuid)
                .layer(middleware!(cors_auth, app_state.clone()))
                .with_state(app_state.clone()),
        )
        .route(
            "/api/login",
            post(user_routes::login)
                .layer(middleware!(cors))
                .with_state(app_state.clone()),
        )
        .route(
            "/api/logout",
            post(user_routes::logout).layer(middleware!(cors)),
        );

    info!("router initialization completed");
    router
}

async fn ping() -> Result<Json<Value>, StatusCode> {
    debug!("ping request received");
    Ok(Json(json!({ "response": "pong"})))
}
