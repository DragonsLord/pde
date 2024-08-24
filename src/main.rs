pub mod commands;
pub mod modules;
pub mod utils;

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    brightness::{BrightnessCommand, BrightnessCommandConfig, BrightnessCommandHandler},
    theme::{ThemeCommand, ThemeCommandConfig, ThemeCommandHandler},
    volume::{VolumeCommand, VolumeCommandConfig, VolumeCommandHandler},
};

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

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }
    let home_dir = env::var("HOME")?;

    // TODO: resolve config from environment
    match &cli.command {
        Commands::Theme(cmd) => ThemeCommandHandler::create(ThemeCommandConfig {
            wallpaper_target_path: Path::new(&home_dir).join(".desk-env/theme/wallpaper.png"),
        })
        .handle(cmd)?,
        Commands::Volume(cmd) => VolumeCommandHandler::create(VolumeCommandConfig {
            step: 2,
            limit: 1.2,
            notification_timeout: 3000,
        })
        .handle(cmd)?,
        Commands::Brightness(cmd) => BrightnessCommandHandler::create(BrightnessCommandConfig {
            step: 2,
            keyboard_device: "asus::kbd_backlight".to_owned(),
            notification_timeout: 3000,
        })
        .handle(cmd)?,
    }

    Ok(())
}
