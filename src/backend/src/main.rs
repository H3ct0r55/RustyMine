use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use tracing::info;

use rustymine_backend::config::AppCfg;
use rustymine_backend::domain::user;
use rustymine_backend::logging;
use rustymine_backend::{cli, interface};

#[derive(Parser, Debug)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Daemon,
    Cli,
}

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging();

    let args = Cli::parse();
    let config = AppCfg::load()?;

    info!(
        "Running RustyMine with args: {:?}, config: {:?}",
        args, config
    );

    match args.command {
        Commands::Daemon => interface::daemon::run_daemon(&config).await?,
        Commands::Cli => cli::run_cli(&config).await?,
    }

    Ok(())
}
