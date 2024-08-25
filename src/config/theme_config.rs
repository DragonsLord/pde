use serde::Deserialize;
use std::path::PathBuf;

use crate::config::parse_utils::ParseUtils;

#[derive(Deserialize, Debug, Default)]
pub struct ThemeConfig {
    #[serde(deserialize_with = "ParseUtils::parse_optional_path")]
    pub wallpaper_store_path: Option<PathBuf>,
}
