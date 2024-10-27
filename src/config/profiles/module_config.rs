use crate::config::parse_utils::ParseUtils;
use anyhow::{bail, Result};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct ModuleConfig {
    pub name: String,
    pub steps: Vec<ModuleStep>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ModuleStep {
    Tool {
        name: String,
        tool: String,
        packages: Vec<String>,
    },
    Zip {
        name: String,
        #[serde(deserialize_with = "ParseUtils::parse_path")]
        extract_zip_to: PathBuf,
        packages: Vec<String>,
    },
    Script {
        name: String,
        script: String,
    },
}

impl ModuleConfig {
    pub fn parse_dir(dir_path: &PathBuf) -> Result<HashMap<String, Self>> {
        if !dir_path.is_dir() {
            bail!("modules_definitions should be a directory path");
        }

        let mut modules = HashMap::<String, Self>::new();
        for entry in fs::read_dir(dir_path)? {
            let path = entry?.path();

            if path.is_dir() {
                continue;
            }

            let module_config = Self::parse(&path)?;
            modules.insert(module_config.name.clone(), module_config);
        }

        Ok(modules)
    }

    pub fn parse(input_path: &PathBuf) -> Result<Self> {
        if !input_path.exists() {
            bail!(
                "{} not found",
                input_path.to_str().expect("valid tools path")
            );
        }

        let config_str = fs::read_to_string(input_path)?;
        let config: Self = toml::from_str(&config_str)?;

        Ok(config)
    }
}
