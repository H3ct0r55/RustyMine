use std::fs::remove_file;

use anyhow::{Error, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::signal;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::{config::AppCfg, infrastructure::db::Db, state::AppState};

pub async fn run_daemon(config: &AppCfg) -> Result<()> {
    info!("Running RustyMine daemon with config: {:#?}", config);
    let db = Db::connect_and_migrate(config).await?;
    let state = AppState::new(config.clone(), db);

    let shutdown = CancellationToken::new();
    let mut tasks: JoinSet<Result<(), anyhow::Error>> = JoinSet::new();
    {
        let token = shutdown.clone();
        let state = state.clone();

        tasks.spawn(async move {
            run_socket_server(state, token).await?;
            Ok::<(), Error>(())
        });
    }
    info!("Daemon is up! Ctrl + C to exit if you are not running systemd");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Daemon: Ctrl+C pressed, shutting down...");
            shutdown.cancel();
        }
    }
    while let Some(res) = tasks.join_next().await {
        res??;
    }
    Ok(())
}

pub async fn run_socket_server(state: AppState, shutdown: CancellationToken) -> Result<()> {
    let _ = remove_file(&state.config.sock_path);

    let listener = UnixListener::bind(&state.config.sock_path)?;
    info!("Socket server listening on {}", &state.config.sock_path);
    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("Socket server: shutdown requested, exiting accept loop");
                break;
            }

            accept_res = listener.accept() => {
                let (stream, _addr) = accept_res?;

                let state = state.clone();
                let shutdown = shutdown.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, state, shutdown).await {
                        error!("Error handling client: {e}");
                    }
                });
            }
        }
    }
    Ok(())
}

async fn handle_client(
    mut stream: UnixStream,
    state: AppState,
    shutdown: CancellationToken,
) -> Result<()> {
    let mut buffer = [0u8; 1024];

    let n = tokio::select! {
        _ = shutdown.cancelled()=> {
            error!("Socket server: shutdown requested while waiting for data");
            return Ok(());
        }
        res = stream.read(&mut buffer) => {
            res?
        }
    };

    if n == 0 {
        return Ok(());
    }

    let msg = String::from_utf8_lossy(&buffer[..n]).to_string();

    let response = format!("Echo from daemon: {msg}");

    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
