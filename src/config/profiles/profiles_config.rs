use super::{module_config::ModuleConfig, tools_config::ToolConfig};
use anyhow::{bail, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
pub struct ProfilesConfig {
    pub tools_definitions: Option<PathBuf>,
    pub modules_definitions: Option<PathBuf>,

    pub default_profile: Option<String>,

    pub profiles: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub tools: HashMap<String, ToolConfig>,
    #[serde(default)]
    pub modules: HashMap<String, ModuleConfig>,
}

impl ProfilesConfig {
    pub fn parse(input_path: &Option<PathBuf>) -> Result<Self> {
        let config_path = Self::resolve_path(input_path);
        if !config_path.exists() {
            bail!("profiles.toml not found");
        }

        let config_str = fs::read_to_string(&config_path)?;
        let mut config: Self = toml::from_str(&config_str)?;

        let config_root_path = config_path.parent().expect("config path has parent dir");
        config.parse_tools_definitions(&config_root_path)?;
        config.parse_modules_definitions(&config_root_path)?;

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
                Path::new(&home_dir).join(".config/pde/install/profiles.toml")
            })
    }

    fn parse_tools_definitions(&mut self, config_root: &Path) -> Result<()> {
        if let Some(path) = &self.tools_definitions {
            let tools = ToolConfig::parse(&config_root.join(path))?;

            self.tools.extend(tools);
        }

        Ok(())
    }

    fn parse_modules_definitions(&mut self, config_root: &Path) -> Result<()> {
        if let Some(path) = &self.modules_definitions {
            let modules = ModuleConfig::parse_dir(&config_root.join(path))?;

            self.modules.extend(modules);
        }

        Ok(())
    }
}
