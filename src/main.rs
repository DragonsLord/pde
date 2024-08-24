pub mod commands;
pub mod modules;
pub mod utils;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::volume::{VolumeCommand, VolumeCommandConfig, VolumeCommandHandler};
use modules::notification::Notification;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Notify {
        #[arg(short, long)]
        message: String,
    },
    Volume(VolumeCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match &cli.command {
        Commands::Notify { message } => Notification::message(message)
            .timeout(5000)
            .sync_group("user-notification")
            .send()?,
        // TODO: resolve config from environment
        Commands::Volume(cmd) => VolumeCommandHandler::create(VolumeCommandConfig {
            step: 2,
            limit: 1.2,
            notification_timeout: 3000,
        })
        .handle(cmd)?,
    }

    Ok(())
}
