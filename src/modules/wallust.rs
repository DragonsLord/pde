use anyhow::Result;
use std::{path::Path, process::Command};

use crate::utils::command_extensions::CommandExtensions;

pub struct Wallust {}
impl Wallust {
    pub fn run(wallpaper_path: &Path) -> Result<()> {
        Command::new("wallust")
            .arg("run")
            .arg(wallpaper_path)
            .pde_run()?;
        Ok(())
    }

    pub fn update_terminal(wallpaper_path: &Path) -> Result<()> {
        Command::new("wallust")
            .arg("run")
            .arg("-u")
            .arg(wallpaper_path)
            .pde_run()?;
        Ok(())
    }
}
