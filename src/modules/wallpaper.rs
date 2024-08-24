use anyhow::Result;
use std::{path::Path, process::Command};

use crate::utils::command_extensions::CommandExtensions;

pub struct Wallpaper {}
impl Wallpaper {
    pub fn set(wallpaper_path: &Path) -> Result<()> {
        Command::killall_if_running("swaybg")?;
        Command::dispatch("swaybg")
            .arg("-i")
            .arg(wallpaper_path)
            .pde_run()?;
        Ok(())
    }
}
