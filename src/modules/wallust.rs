use anyhow::Result;
use std::{
    path::Path,
    process::{Command, Stdio},
};

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
            .args(["run", "--skip-templates", "--update-current", "--quiet"])
            .arg(wallpaper_path)
            .stdout(Stdio::inherit())
            .pde_run()?;

        Ok(())
    }
}
