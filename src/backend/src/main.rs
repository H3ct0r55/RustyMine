use std::net::SocketAddr;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use axum::Router;
use chrono::Utc;
use rustymine::domain::server::Server;
use rustymine::domain::supervisor::{ServerEvent, ServerState};
use rustymine::infrastructure::config::AppCfg;
use rustymine::infrastructure::state::AppState;
use rustymine::interface::api::build_api_router;
use rustymine::utils::validation::slugify;
use tokio::io::Lines;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::process::{Child, ChildStdout};
use tokio::signal::{self, ctrl_c};
use tokio::time::sleep;
use tokio::{main, process::Command};
use tracing::info;
use tracing_subscriber::EnvFilter;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    info!("Starting RustyMine");
    let config = AppCfg::new();
    let state = Arc::new(AppState::new(&config).await);

    let app = build_api_router(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    info!(%addr, "Listening");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    /*let server = Server {
        id: 1,
        name: "Test Server".to_string(),
        slug: slugify("Test Server"),
        is_active: true,
        path: "/home/hector/mc_svr_tmp/".to_string(),
        jar_path: "server.jar".to_string(),
        j_max_mem: "8G".to_string(),
        j_min_mem: "4G".to_string(),
        created_at: Utc::now(),
        mc_type: "vanilla".to_string(),
        mc_version: "1.21.10".to_string(),
        updated_at: Utc::now(),
    };

    state.supervisor.start_server(server.clone()).await?;

    if let Some(mut rx) = state.supervisor.subscribe(server.id).await {
        tokio::spawn(async move {
            while let Ok(event) = rx.recv().await {
                match event {
                    ServerEvent::OutputLine { source, line } => {
                        println!("[{} {:?}] {}", server.slug, source, line);
                    }
                    ServerEvent::StateChanged(state) => {
                        println!("[{} STATE] {:?}", server.slug, state);
                    }
                }
            }
        });
    } else {
        eprintln!("No ManagedServer found for id {}", server.id);
    }

    signal::ctrl_c().await?;
    state
        .supervisor
        .send_command(server.id, "/stop".to_string())
        .await?;

    // --- 6) Wait until we see Exited/Crashed for that server ---
    // We create a fresh receiver so we can specifically watch for the exit event.
    if let Some(mut exit_rx) = state.supervisor.subscribe(server.id).await {
        while let Ok(event) = exit_rx.recv().await {
            if let ServerEvent::StateChanged(st) = event {
                match st {
                    ServerState::Exited(code) => {
                        println!("Server exited cleanly with code {code}");
                        break;
                    }
                    ServerState::Crashed(code) => {
                        println!("Server crashed with code {code}");
                        break;
                    }
                    _ => {
                        // ignore intermediate states
                    }
                }
            }
        }
    } else {
        // If for some reason we couldn't subscribe, just give it a moment to die
        sleep(Duration::from_secs(5)).await;
    }*/
    Ok(())
}
