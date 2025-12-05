use crate::{core::user_routines, domain::user::InternalUser, prelude::*};
use std::sync::Arc;

use axum::{
    Extension,
    extract::{MatchedPath, Request, State},
    http::{self, Method, StatusCode, header::AUTHORIZATION},
    middleware::{Next, from_fn_with_state},
    response::Response,
};

use tower_http::cors::{Any, CorsLayer};
use tracing::debug;

use crate::{auth::verify_jwt, infra::db, state::AppState};

pub async fn auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let request_method = req.method().clone();
    let request_path = req.uri().path().to_string();

    debug!(method = ?request_method, path = request_path, "authenticate request started");

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(StatusCode::FORBIDDEN)?;

    let auth_header = auth_header.to_str().map_err(|e| {
        error!(error = %e, method = ?request_method, path = request_path, "authorization header parse failed");
        StatusCode::FORBIDDEN
    })?;

    let mut parts = auth_header.split_whitespace();
    let (scheme, token) = match (parts.next(), parts.next()) {
        (Some(scheme), Some(token)) if scheme.eq_ignore_ascii_case("bearer") => (scheme, token),
        _ => {
            warn!(method = ?request_method, path = request_path, "authorization header missing bearer token");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let token_data = verify_jwt(token.to_string())?;
    let username = &token_data.claims.username;

    let current_user = match user_routines::get_by_username(state, username)
        .await
        .map_err(|e| {
            error!(error = %e, method = ?request_method, path = request_path, username, "fetch user for auth failed");
            return StatusCode::INTERNAL_SERVER_ERROR;
        })? {
        Some(user) => user,
        None => {
            error!(method = ?request_method, path = request_path, username, "authenticated user missing in database");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}

pub fn cors() -> CorsLayer {
    debug!("build cors layer");
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION])
}

pub async fn permissions(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<InternalUser>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let request_method = req.method().clone();
    let request_path = req.uri().path().to_string();

    debug!(method = ?request_method, path = request_path, "permissions request started");
    debug!("Calling user {}", user.username.clone());
    let method: Method = req.method().clone();

    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_string())
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    match state
        .config
        .route_allows(&method, path.as_str(), user.permissions.clone())
    {
        true => return Ok(next.run(req).await),
        false => return Err(StatusCode::UNAUTHORIZED),
    };
}
