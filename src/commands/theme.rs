use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;

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
    SelectWallpaper,
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
    wallpapers_dirs: Vec<PathBuf>,
    theme_selector_config_path: Option<PathBuf>,
    on_init_commands: Vec<String>,
}

impl ThemeCommandHandler {
    pub fn create(config: &Config) -> Self {
        Self {
            wallpaper_target_path: Self::resolve_wallpaper_target_path(config),
            theme_variables_path: Self::resolve_theme_variables_path(config),
            on_init_commands: config.theme.on_init.clone(),
            wallpapers_dirs: Self::resolve_wallpapers_folder(config),
            theme_selector_config_path: config.theme.selector.config_path.clone(),
        }
    }

    pub fn handle(self, cmd: &ThemeCommand) -> Result<()> {
        match &cmd.command {
            ThemeSubcommands::SelectWallpaper => {
                self.select_wallpaper()?;
            }
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

    fn select_wallpaper(&self) -> Result<()> {
        let mut binding = Command::new("rofi");
        let menu_cmd = binding.args(["-dmenu", "-show-icons", "-p", " ", "-format", "i"]);

        if let Some(config_path) = &self.theme_selector_config_path {
            menu_cmd.args(["-config", config_path.to_str().expect("valid config path")]);
        }

        let mut menu = menu_cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut all_items: Vec<PathBuf> = vec![];
        {
            let mut menu_in = menu.stdin.take().expect("piped rofi menu stdin");

            for dir in self.wallpapers_dirs.iter() {
                let wallpaper_list = dir.read_dir()?;
                for wallpaper in wallpaper_list {
                    let path = wallpaper?.path();
                    if !path.is_file() {
                        continue;
                    }
                    if let Some(filename) = path.file_name() {
                        writeln!(
                            menu_in,
                            "{}\0icon\x1f{}",
                            filename.to_str().unwrap(),
                            path.display(),
                        )?;
                        all_items.push(path);
                    }
                }
            }

            menu_in.flush()?;
        }

        let output = menu.wait_with_output()?;
        let output_string = String::from_utf8_lossy(&output.stdout);
        if output_string.is_empty() {
            return Ok(());
        }

        let idx = usize::from_str(&output_string.trim())?;
        let selected_wallpaper = all_items[idx].clone();
        println!("setting '{}' wallpaper...", &selected_wallpaper.display());

        self.set_wallpaper(&selected_wallpaper, true)?;

        return Ok(());
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

    fn resolve_wallpapers_folder(config: &Config) -> Vec<PathBuf> {
        if config.theme.selector.wallpapers_dirs.is_empty() {
            return vec!["~/wallpapers".into()];
        }
        config.theme.selector.wallpapers_dirs.to_owned()
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
