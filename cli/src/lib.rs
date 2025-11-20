mod config;

use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::config::Config;

#[derive(Parser)]
struct Cli {
    #[clap(
        short,
        long,
        env = "NODESCOPE_CONFIG",
        default_value = "nodescope.yml",
        value_name = "FILE"
    )]
    config: PathBuf,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run,
}

pub async fn run() -> anyhow::Result<()> {
    // Initialize tracing subscriber to output logs to console
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Run) {
        Commands::Run => {
            let config = Config::init(cli.config)?;
            run_app(config).await?;
        }
    }

    Ok(())
}

async fn run_app(config: Config) -> anyhow::Result<()> {
    let app = app::NodeScopeApp::new(&config.database_path).await?;

    tokio::try_join!(
        async {
            proxy::run(config.proxy.clone(), app.clone())
                .await
                .context("proxy server error")
        },
        async {
            server::run(config.server.clone(), app.clone())
                .await
                .context("server error")
        }
    )?;

    Ok(())
}
