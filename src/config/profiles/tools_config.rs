use anyhow::{bail, Result};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct ToolConfig {
    pub cmd: String,
    #[serde(default)]
    pub batching: bool,
}

impl ToolConfig {
    pub fn parse(input_path: &PathBuf) -> Result<HashMap<String, ToolConfig>> {
        if !input_path.exists() {
            bail!(
                "{} not found",
                input_path.to_str().expect("valid tools path")
            );
        }

        let config_str = fs::read_to_string(input_path)?;
        let config: HashMap<String, ToolConfig> = toml::from_str(&config_str)?;

        Ok(config)
    }
}
