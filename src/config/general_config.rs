use serde::Deserialize;
use std::path::PathBuf;

use super::{defaults::Defaults, parse_utils::ParseUtils};

#[derive(Deserialize, Debug)]
pub struct GeneralConfig {
    #[serde(deserialize_with = "ParseUtils::parse_path")]
    #[serde(default = "Defaults::resource_root_dir")]
    pub resource_root_dir: PathBuf,

    #[serde(default = "Defaults::notification_timeout_ms")]
    pub notification_timeout_ms: i32,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            resource_root_dir: Defaults::resource_root_dir(),
            notification_timeout_ms: Defaults::notification_timeout_ms(),
        }
    }
}
