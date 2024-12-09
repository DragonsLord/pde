use serde::Deserialize;
use std::path::PathBuf;

use crate::config::parse_utils::ParseUtils;

#[derive(Deserialize, Debug, Default)]
pub struct ThemeConfig {
    #[serde(default, deserialize_with = "ParseUtils::parse_optional_path")]
    pub wallpaper_store_path: Option<PathBuf>,
    #[serde(default, deserialize_with = "ParseUtils::parse_optional_path")]
    pub theme_variables_path: Option<PathBuf>,
    pub on_init: Vec<String>,
}
