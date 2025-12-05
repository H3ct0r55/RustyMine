use std::{collections::HashMap, sync::Arc};

use anyhow::{Ok, Result};
use axum::http::Method;
use rustymine_daemon::{
    config::AppCfg,
    domain::user_prems::UserActions,
    router,
    state::{AppState, check_root},
};
use tracing::{Level, info};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const BUILD_DATE: &str = env!("BUILD_DATE");
pub const GIT_HASH: &str = env!("GIT_HASH");
pub const GIT_SUFFIX: &str = env!("GIT_SUFFIX");

pub const ASCII_LOGO: &str = "
██████╗ ██╗   ██╗███████╗████████╗██╗   ██╗███╗   ███╗██╗███╗   ██╗███████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝╚██╗ ██╔╝████╗ ████║██║████╗  ██║██╔════╝
██████╔╝██║   ██║███████╗   ██║    ╚████╔╝ ██╔████╔██║██║██╔██╗ ██║█████╗  
██╔══██╗██║   ██║╚════██║   ██║     ╚██╔╝  ██║╚██╔╝██║██║██║╚██╗██║██╔══╝  
██║  ██║╚██████╔╝███████║   ██║      ██║   ██║ ╚═╝ ██║██║██║ ╚████║███████╗
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝   ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝";

pub const BUILD_MODE: &str = if cfg!(debug_assertions) {
    "development"
} else {
    "release"
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", ASCII_LOGO);
    println!("\nStarting {} v{}-{}", APP_NAME, APP_VERSION, BUILD_MODE);
    println!("Git revision: {}{}", GIT_HASH, GIT_SUFFIX);
    println!("Built on:     {}\n", BUILD_DATE);

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!(
        app = APP_NAME,
        version = APP_VERSION,
        build_mode = BUILD_MODE,
        "starting application"
    );
    info!(
        git_hash = GIT_HASH,
        git_suffix = GIT_SUFFIX,
        build_date = BUILD_DATE,
        "build metadata"
    );

    let db_path: String = "postgres://rustymine:minecraft@localhost:5432/rustymine_dev".to_string();
    let mut config = AppCfg {
        db_path: db_path.clone(),
        route_perms: HashMap::new(),
    };

    config.insert_route_perms(
        Method::GET,
        "/api/users",
        false,
        vec![UserActions::ManageUsers],
    );

    let state = Arc::new(AppState::new(config).await);
    check_root(state.clone()).await;

    let app_result = router::init_router(state.clone()).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!(listen_addr = "0.0.0.0:3000", "http server binding started");

    axum::serve(listener, app_result).await?;
    info!("http server stopped");
    Ok(())
}
