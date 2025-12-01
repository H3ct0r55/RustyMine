use crate::{core::user_routines, prelude::*};
use std::sync::Arc;

use axum::{
    extract::{Request, State},
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

    // 1) Extract Authorization header
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(StatusCode::FORBIDDEN)?; // no header at all

    let auth_header = auth_header.to_str().map_err(|e| {
        error!(error = %e, method = ?request_method, path = request_path, "authorization header parse failed");
        StatusCode::FORBIDDEN
    })?;

    // 2) Expect "Bearer <token>"
    let mut parts = auth_header.split_whitespace();
    let (scheme, token) = match (parts.next(), parts.next()) {
        (Some(scheme), Some(token)) if scheme.eq_ignore_ascii_case("bearer") => (scheme, token),
        _ => {
            // either wrong scheme or missing token
            warn!(method = ?request_method, path = request_path, "authorization header missing bearer token");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 3) Verify JWT
    let token_data = verify_jwt(token.to_string())?; // verify_jwt(&str) -> Result<TokenData<AuthClaims>, StatusCode>
    let username = &token_data.claims.username;

    // 4) Load current user from DB
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
    // 5) Attach user to request extensions so handlers can grab it
    req.extensions_mut().insert(current_user);

    // 6) Continue down the stack
    Ok(next.run(req).await)
}

pub fn cors() -> CorsLayer {
    debug!("build cors layer");
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION])
}
