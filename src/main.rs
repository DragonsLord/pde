pub mod commands;
pub mod utils;

use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand};
use commands::volume::{execute_volume_command, VolumeCommand};
use utils::notification::{send_notification, Notification};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Notify {
        #[arg(short, long)]
        message: String,
    },
    Volume(VolumeCommand),
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Notify { message }) => {
            send_notification(
                Notification::message(message)
                    .timeout(5000)
                    .sync_group("user-notification"),
            );
        }
        Some(Commands::Volume(cmd)) => execute_volume_command(cmd),
        None => {
            Cli::command().print_long_help().expect("help failed");
        }
    }
}
