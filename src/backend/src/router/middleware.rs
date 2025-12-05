use crate::{core::user_routines, domain::user::InternalUser, prelude::*};
use std::sync::Arc;

use axum::{
    Extension,
    extract::{MatchedPath, Request, State},
    http::{self, Method, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use axum_extra::extract::CookieJar;
use tower_http::cors::{Any, CorsLayer};
use tracing::debug;

use crate::{auth::verify_jwt, state::AppState};

pub async fn auth(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    debug!(?method, path, "authenticate request started");

    // 1) Try JWT from cookie first
    let token_from_cookie = jar
        .get("auth_token")
        .map(|cookie| cookie.value().to_owned());

    // 2) If no cookie, fall back to Authorization: Bearer ...
    let token = match token_from_cookie {
        Some(t) => t,
        None => {
            let auth_header = req
                .headers()
                .get(http::header::AUTHORIZATION)
                .ok_or(StatusCode::FORBIDDEN)?;

            let auth_header = auth_header.to_str().map_err(|e| {
                error!(
                    error = %e,
                    ?method,
                    path,
                    "authorization header parse failed"
                );
                StatusCode::FORBIDDEN
            })?;

            let mut parts = auth_header.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(scheme), Some(token)) if scheme.eq_ignore_ascii_case("bearer") => {
                    token.to_owned()
                }
                _ => {
                    warn!(?method, path, "authorization header missing bearer token");
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    };

    // 3) Verify JWT
    let token_data = verify_jwt(token).map_err(|e| {
        warn!(error = %e, ?method, path, "invalid jwt");
        StatusCode::UNAUTHORIZED
    })?;
    let username = &token_data.claims.username;

    // 4) Load user from DB
    let current_user = user_routines::get_by_username(state, username)
        .await
        .map_err(|e| {
            error!(
                error = %e,
                ?method,
                path,
                username,
                "fetch user for auth failed"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            error!(
                ?method,
                path, username, "authenticated user missing in database"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 5) Attach user to request extensions
    req.extensions_mut().insert(current_user);

    // 6) Continue the chain
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
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let request_method = req.method().clone();
    let request_path = req.uri().path().to_string();

    debug!(method = ?request_method, path = request_path, "permissions request started");
    debug!("Calling user {}", user.username.clone());
    if user.permissions.root {
        return Ok(next.run(req).await);
    }

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
        true => Ok(next.run(req).await),
        false => Err(StatusCode::UNAUTHORIZED),
    }
}
