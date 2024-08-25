use anyhow::Result;
use brightness_config::BrightnessConfig;
use general_config::GeneralConfig;
use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use theme_config::ThemeConfig;
use volume_config::VolumeConfig;

use serde::Deserialize;

pub mod brightness_config;
pub mod defaults;
pub mod general_config;
pub mod parse_utils;
pub mod theme_config;
pub mod volume_config;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub volume: VolumeConfig,
    #[serde(default)]
    pub brightness: BrightnessConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
}

impl Config {
    pub fn parse(input_path: Option<PathBuf>) -> Result<Self> {
        let config_path = Self::resolve_path(input_path);
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let config_str = read_to_string(config_path)?;
        let config: Self = toml::from_str(&config_str)?;

        Ok(config)
    }

    fn resolve_path(input_path: Option<PathBuf>) -> PathBuf {
        if let Some(path) = input_path {
            return path;
        }

        env::var("PDE_CONFIG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home_dir = env::var("HOME").expect("HOME variable is not set");
                Path::new(&home_dir).join(".config/pde/config.toml")
            })
    }
}
