use anyhow::Result;
use tracing::info;

use crate::config::AppCfg;

pub async fn run_cli(config: &AppCfg) -> Result<()> {
    info!("CLI is not yet implemented. Config: {:?}", config);
    println!("RustyMine CLI mode is not yet implemented.");
    Ok(())
}
