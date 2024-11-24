use std::process::Command;

use anyhow::Result;

use crate::utils::command_extensions::CommandExtensions;

pub struct Flatpak {}

impl Flatpak {
    pub fn is_running(app: &str) -> Result<bool> {
        let output = Command::new("flatpak")
            .arg("ps")
            .arg("--columns=application")
            .pde_run()?;

        Ok(String::from_utf8(output)?
            .split('\n')
            .skip(1)
            .filter(|a| a.eq(&app))
            .count()
            > 0)
    }

    pub fn kill(app: &str) -> Result<()> {
        Command::new("flatpak").arg("kill").arg(app).pde_run()?;
        Ok(())
    }
}
