use axum::{
    extract::Request,
    http::{Method, header::AUTHORIZATION},
    middleware::Next,
    response::IntoResponse,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::debug;

pub async fn auth(request: Request, next: Next) -> impl IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().path().to_owned();

    debug!(%method, uri, "auth middleware started");
    let response = next.run(request).await;
    let status = response.status();
    debug!(%method, uri, %status, "auth middleware completed");
    response
}

pub fn cors() -> CorsLayer {
    debug!("build cors layer");
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION])
}
