use anyhow::{bail, Result};
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::config::parse_utils::ParseUtils;

#[derive(Deserialize, Debug)]
pub struct ProfilesConfig {
    pub default_profile: Option<String>,
    pub profiles: HashMap<String, Vec<String>>,
    pub tools: HashMap<String, ToolConfig>,
    pub modules: HashMap<String, Vec<ModuleConfig>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ToolConfig {
    pub cmd: String,
    #[serde(default)]
    pub batching: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ModuleConfig {
    Tool {
        tool: String,
        packages: Vec<String>,
    },
    Zip {
        #[serde(deserialize_with = "ParseUtils::parse_path")]
        extract_zip_to: PathBuf,
        packages: Vec<String>,
    },
    Cmd {
        cmd: Vec<String>,
    },
}

impl ProfilesConfig {
    pub fn parse(input_path: &Option<PathBuf>) -> Result<Self> {
        let config_path = Self::resolve_path(input_path);
        if !config_path.exists() {
            bail!("profiles.toml not found");
        }

        let config_str = fs::read_to_string(config_path)?;
        let config: Self = toml::from_str(&config_str)?;

        Ok(config)
    }

    fn resolve_path(input_path: &Option<PathBuf>) -> PathBuf {
        if let Some(path) = input_path {
            return path.to_owned();
        }

        env::var("PDE_PROFILES_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home_dir = env::var("HOME").expect("HOME variable is not set");
                Path::new(&home_dir).join(".config/pde/profiles.toml")
            })
    }
}
