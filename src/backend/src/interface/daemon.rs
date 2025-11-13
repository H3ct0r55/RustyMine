use anyhow::Result;
use tokio::signal;
use tracing::{info, warn};

use crate::config::AppCfg;

pub async fn run_daemon(config: &AppCfg) -> Result<()> {
    info!("Running RustyMine daemon with config: {:?}", config);
    info!("Daemon is up! Ctrl + C to exit if you are not running systemd");
    if let Err(e) = signal::ctrl_c().await {
        warn!("Failed to listen to shutdown signal: {e}");
    }
    Ok(())
}
