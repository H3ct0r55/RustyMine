use crate::domain::user::{NewUser, SafeUser, User};
use crate::interface::protocol::{DaemonRequest, DaemonResponse, ResponseStatus};
use crate::state::AppState;
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
use tracing::warn;

pub async fn route_request(state: &AppState, req: &DaemonRequest) -> Result<DaemonResponse> {
    match req.command.as_str() {
        "ping" => {
            let data = json!({ "message": "pong" });
            Ok(DaemonResponse::ok(req, data))
        }

        "user.list" => {
            let list_res = state.user_repo.list_all().await;

            let result = match list_res {
                Ok(users) => {
                    let safe_users: Vec<SafeUser> = users.into_iter().map(SafeUser::from).collect();

                    DaemonResponse::ok(req, json!({ "users": safe_users }))
                }

                Err(e) => DaemonResponse::error(
                    Some(req),
                    "NO_USERS",
                    "No users found in DB",
                    Some(json!({ "error": e.to_string() })),
                ),
            };

            Ok(result)
        }

        "user.create" => {
            let newuser: NewUser = serde_json::from_value(req.args.clone())?;
            let result = match state.user_repo.create(newuser).await {
                Ok(addeduser) => {
                    let safe_user: SafeUser = SafeUser::from(addeduser);

                    DaemonResponse::ok(req, json!({ "user": safe_user }))
                }
                Err(e) => DaemonResponse::error(
                    Some(req),
                    "FAILED_TO_CREATE",
                    "Failed to create user",
                    Some(json!({ "error": e.to_string() })),
                ),
            };

            Ok(result)
        }

        other => {
            let details = json!({ "command": other });
            Ok(DaemonResponse::error(
                Some(req),
                "UNKNOWN_COMMAND",
                "Unknown command",
                Some(details),
            ))
        }
    }
}
