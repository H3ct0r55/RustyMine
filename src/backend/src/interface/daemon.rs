use anyhow::Result;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, warn};

use crate::{
    config::AppCfg,
    domain::{
        repository::UserRepository,
        user::{NewUser, User, UserRole},
    },
    infrastructure::{db::Db, sqlite_user_repo::SqliteUserRepository},
    state::AppState,
    utils::clean_error,
};

pub async fn run_daemon(config: &AppCfg) -> Result<()> {
    info!("Running RustyMine daemon with config: {:#?}", config);
    let db = Db::connect_and_migrate(config).await?;
    let state = AppState::new(config.clone(), db);
    let state = Arc::new(state);
    info!("App state configued as: {:#?}", state);
    info!("Daemon is up! Ctrl + C to exit if you are not running systemd");

    let user_repo = SqliteUserRepository::new(state.db.pool.clone());

    let new_user = NewUser {
        username: "h3cx2".to_string(),
        password_hash: "TEST HASH".to_string(),
        role: UserRole::Admin,
        email: Some("hector@h3cx.dev".to_string()),
    };

    let created = user_repo.create(new_user).await;
    match created {
        Ok(ref user) => {
            info!("Created user: {:#?}", user);
        }
        Err(ref e) => {
            error!("Create user failed: {:?}", clean_error(e));
        }
    };

    match created {
        Ok(ref user) => {
            let fetched = user_repo.get_by_id(user.id).await?;
            info!("Fetched user by id: {:#?}", fetched);
        }
        Err(e) => {
            error!("User struct does not exist: {:?}", clean_error(e));
        }
    }

    let all = user_repo.list_all().await?;
    info!("Listing all users: {:#?}", all);

    if let Err(e) = signal::ctrl_c().await {
        warn!("Failed to listen to shutdown signal: {e}");
    }
    Ok(())
}
