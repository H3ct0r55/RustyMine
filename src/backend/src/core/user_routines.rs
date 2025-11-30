use crate::prelude::*;
use std::sync::Arc;

use axum::http::StatusCode;
use validator::Validate;

use crate::{
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};

pub async fn create(state: Arc<AppState>, new_user: NewUser) -> Result<User, StatusCode> {
    if let Err(_) = new_user.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let internal = InternalNewUser::try_from(new_user).map_err(|e| {
        error!("Conversion to InternalUser failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    todo!("Hook up return once db setup ready");
}
