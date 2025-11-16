use anyhow::{Error, Result};
use serde_json::json;
use std::fs::remove_file;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::signal;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::interface::router::route_request;

use crate::interface::protocol::{
    DaemonError, DaemonMessage, DaemonRequest, DaemonResponse, ResponseStatus,
};
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
    stream: UnixStream,
    state: AppState,
    shutdown: CancellationToken,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Read one line from client, or exit if shutdown
    let n = tokio::select! {
        _ = shutdown.cancelled() => {
            error!("Socket server: shutdown requested while waiting for data");
            return Ok(());
        }
        res = reader.read_line(&mut line) => {
            res?
        }
    };

    if n == 0 {
        // Client closed connection
        return Ok(());
    }

    let line = line.trim_end_matches(&['\n', '\r'][..]);

    if line.is_empty() {
        let resp = DaemonResponse::error(
            None, // we never got a valid request
            "EMPTY_REQUEST",
            "Empty request payload",
            None,
        );
        let json_resp = serde_json::to_string(&resp)?;
        writer.write_all(json_resp.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        return Ok(());
    }

    // Try to parse JSON into DaemonRequest
    let req: DaemonRequest = match serde_json::from_str(line) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to parse JSON from client: {e}; raw={line}");

            let resp = DaemonResponse::error(
                None, // still no valid request
                "INVALID_JSON",
                "Failed to parse JSON request",
                Some(json!({ "error": e.to_string() })),
            );
            let json_resp = serde_json::to_string(&resp)?;
            writer.write_all(json_resp.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            return Ok(());
        }
    };

    info!(command = %req.command, "Handling daemon request");

    // Route valid request
    let resp = match route_request(&state, &req).await {
        Ok(r) => r,
        Err(e) => {
            error!("Internal error while handling request: {e}");
            DaemonResponse::error(
                Some(&req), // we *do* have a valid request here
                "INTERNAL_ERROR",
                "Internal server error",
                Some(json!({ "error": e.to_string() })),
            )
        }
    };

    let json_resp = serde_json::to_string(&resp)?;
    writer.write_all(json_resp.as_bytes()).await?;
    writer.write_all(b"\n").await?;

    Ok(())
}
