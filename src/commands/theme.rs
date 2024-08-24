use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::modules::{wallpaper::Wallpaper, wallust::Wallust};
use crate::utils::command_extensions::CommandExtensions;
use crate::utils::image_utils::save_as_png;

#[derive(Args)]
pub struct ThemeCommand {
    #[command(subcommand)]
    command: ThemeSubcommands,
}

#[derive(Subcommand)]
enum ThemeSubcommands {
    SetWallpaper { wallpaper: PathBuf },
    InitWallpaper,
    InitTerminal,
}

pub struct ThemeCommandConfig {
    pub wallpaper_target_path: PathBuf,
}

pub struct ThemeCommandHandler {
    config: ThemeCommandConfig,
}

impl ThemeCommandHandler {
    pub fn create(config: ThemeCommandConfig) -> Self {
        Self { config }
    }
    pub fn handle(self, cmd: &ThemeCommand) -> Result<()> {
        match &cmd.command {
            ThemeSubcommands::SetWallpaper { wallpaper } => {
                self.set_wallpaper(wallpaper)?;
            }
            ThemeSubcommands::InitWallpaper => {
                self.init_wallpaper()?;
            }
            ThemeSubcommands::InitTerminal => {
                self.init_terminal()?;
            }
        }
        Ok(())
    }

    fn set_wallpaper(self, wallpaper_path: &Path) -> Result<()> {
        save_as_png(wallpaper_path, &self.config.wallpaper_target_path)?;
        Wallust::run(wallpaper_path)?;
        Wallpaper::set(&self.config.wallpaper_target_path)?;

        // TODO: make configurable
        Self::reload("waybar")?;
        Self::reload("swaync")?;

        Ok(())
    }

    fn init_wallpaper(self) -> Result<()> {
        Wallpaper::set(&self.config.wallpaper_target_path)?;
        Ok(())
    }

    fn init_terminal(self) -> Result<()> {
        Wallust::update_terminal(&self.config.wallpaper_target_path)?;
        Ok(())
    }

    fn reload(program: &str) -> Result<()> {
        Command::killall_if_running(program)?;
        Command::dispatch(program).pde_run()?;
        Ok(())
    }
}
