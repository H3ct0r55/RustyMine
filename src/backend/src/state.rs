use std::{collections::HashMap, process::exit, sync::Arc};

use crate::{
    core,
    domain::{
        user::{InternalNewUser, NewUser},
        user_prems::UserActions,
    },
    prelude::*,
};

use sqlx::PgPool;

use crate::{config::AppCfg, infra::db};

pub struct AppState {
    pub db_pool: PgPool,
    pub config: AppCfg,
}

impl AppState {
    pub async fn new(config: AppCfg) -> Self {
        debug!("init app state");
        debug!("establish database connection");
        let db_pool = db::connect(&config.db_path)
            .await
            .map_err(|e| {
                error!(error = %e, "connect to database failed");
                exit(20);
            })
            .unwrap();

        debug!("run database migrations");
        db::migrate(&db_pool)
            .await
            .map_err(|e| {
                error!(error = %e, "database migration failed");
                exit(22);
            })
            .unwrap();
        info!("database ready after connect and migrate");

        Self {
            db_pool: db_pool,
            config: config,
        }
    }
}

pub async fn check_root(state: Arc<AppState>) {
    let root_exists = db::user::exists_by_username(&state.db_pool, "root")
        .await
        .map_err(|e| {
            error!(error = %e, "check root exists failed");
            exit(23);
        })
        .unwrap();

    if !root_exists {
        info!("No root user found in db, creating one");
        let new_root = NewUser::new_root();
        core::user_routines::create(state, new_root)
            .await
            .map_err(|e| {
                error!(error = %e, "create root failed");
                exit(24);
            })
            .unwrap();
        info!("New default root created username: root, password: rootpassword");
    }
}
