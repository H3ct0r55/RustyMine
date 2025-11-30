use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use tracing::{debug, error, info, warn};
use validator::Validate;

use crate::{
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    if let Err(_) = new_user.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let internal = InternalNewUser::try_from(new_user).map_err(|e| {
        error!("Conversion to InternalUser failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    todo!("Hook up return once db setup ready");
}
