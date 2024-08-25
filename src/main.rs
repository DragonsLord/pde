pub mod commands;
pub mod config;
pub mod modules;
pub mod utils;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    brightness::{BrightnessCommand, BrightnessCommandHandler},
    theme::{ThemeCommand, ThemeCommandHandler},
    volume::{VolumeCommand, VolumeCommandHandler},
};
use config::Config;

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
    Volume(VolumeCommand),
    Brightness(BrightnessCommand),
    Theme(ThemeCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::parse(cli.config)?;
    match &cli.command {
        Commands::Theme(cmd) => ThemeCommandHandler::create(&config).handle(cmd)?,
        Commands::Volume(cmd) => VolumeCommandHandler::create(&config).handle(cmd)?,
        Commands::Brightness(cmd) => BrightnessCommandHandler::create(&config).handle(cmd)?,
    }

    Ok(())
}
