use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::Config;
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
}

pub struct ThemeCommandHandler {
    wallpaper_target_path: PathBuf,
}

impl ThemeCommandHandler {
    pub fn create(config: &Config) -> Self {
        Self {
            wallpaper_target_path: Self::resolve_wallpaper_target_path(config),
        }
    }

    pub fn handle(self, cmd: &ThemeCommand) -> Result<()> {
        match &cmd.command {
            ThemeSubcommands::SetWallpaper { wallpaper } => {
                self.set_wallpaper(wallpaper)?;
            }
            ThemeSubcommands::InitWallpaper => {
                self.init_wallpaper()?;
            }
        }
        Ok(())
    }

    fn set_wallpaper(self, wallpaper_path: &Path) -> Result<()> {
        save_as_png(wallpaper_path, &self.wallpaper_target_path)?;
        Wallust::run(wallpaper_path)?;
        Wallpaper::set(&self.wallpaper_target_path)?;

        // TODO: make configurable
        Self::reload("waybar")?;
        Self::reload("swaync")?;

        Ok(())
    }

    fn init_wallpaper(self) -> Result<()> {
        Wallpaper::set(&self.wallpaper_target_path)?;
        Ok(())
    }

    fn reload(program: &str) -> Result<()> {
        Command::killall_if_running(program)?;
        Command::dispatch(program).pde_run()?;
        Ok(())
    }

    fn resolve_wallpaper_target_path(config: &Config) -> PathBuf {
        match &config.theme.wallpaper_store_path {
            Some(path) => path.to_owned(),
            None => {
                let mut path = config.general.resource_root_dir.clone();
                path.push("theme/wallpaper.png");
                path
            }
        }
    }
}
