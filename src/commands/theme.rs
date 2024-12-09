use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
    SetWallpaper {
        wallpaper: PathBuf,
        #[arg(long)]
        #[clap(default_value_t = false)]
        no_reload: bool,
    },
    InitWallpaper,
}

pub struct ThemeCommandHandler {
    wallpaper_target_path: PathBuf,
    theme_variables_path: PathBuf,
    on_init_commands: Vec<String>,
}

impl ThemeCommandHandler {
    pub fn create(config: &Config) -> Self {
        Self {
            wallpaper_target_path: Self::resolve_wallpaper_target_path(config),
            theme_variables_path: Self::resolve_theme_variables_path(config),
            on_init_commands: config.theme.on_init.clone(),
        }
    }

    pub fn handle(self, cmd: &ThemeCommand) -> Result<()> {
        match &cmd.command {
            ThemeSubcommands::SetWallpaper {
                wallpaper,
                no_reload,
            } => {
                self.set_wallpaper(wallpaper, !no_reload)?;
            }
            ThemeSubcommands::InitWallpaper => {
                self.init_wallpaper()?;
            }
        }
        Ok(())
    }

    fn set_wallpaper(&self, wallpaper_path: &Path, reload: bool) -> Result<()> {
        save_as_png(wallpaper_path, &self.wallpaper_target_path)?;
        Wallust::run(wallpaper_path)?;

        if reload {
            self.init_wallpaper()?;

            // TODO: make configurable
            Self::reload("waybar")?;
            Self::reload("swaync")?;
        }

        Ok(())
    }

    fn init_wallpaper(&self) -> Result<()> {
        Wallpaper::set(&self.wallpaper_target_path)?;
        self.run_on_init_handlers()?;
        Ok(())
    }

    fn run_on_init_handlers(&self) -> Result<()> {
        let variables = self.get_theme_variables()?;

        for cmd in &self.on_init_commands {
            let mut result_cmd = cmd.to_owned();
            for (key, value) in variables.iter() {
                let pattern = r"\{\{\s*".to_owned() + key + r"\s*\}\}";
                result_cmd = Regex::new(&pattern)?
                    .replace_all(&result_cmd, value)
                    .to_string();
            }

            println!("Running '{}' command:", &result_cmd);

            Command::from_string(&result_cmd)?
                .stdout(Stdio::inherit())
                .pde_run()?;
        }

        Ok(())
    }

    fn get_theme_variables(&self) -> Result<HashMap<String, String>> {
        if self.theme_variables_path.exists() {
            let str_content = read_to_string(&self.theme_variables_path)?;
            let variables = toml::from_str(&str_content)?;
            return Ok(variables);
        }
        return Ok(HashMap::new());
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

    fn resolve_theme_variables_path(config: &Config) -> PathBuf {
        match &config.theme.theme_variables_path {
            Some(path) => path.to_owned(),
            None => {
                let mut path = config.general.resource_root_dir.clone();
                path.push("theme/variables.toml");
                path
            }
        }
    }
}
