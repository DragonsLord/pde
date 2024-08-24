use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct ThemeConfig {
    pub wallpaper_store_path: Option<PathBuf>,
}
