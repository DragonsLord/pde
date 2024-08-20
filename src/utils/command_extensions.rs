use anyhow::{bail, Result};
use std::process::Command;

pub trait CommandExtensions {
    fn pde_run(&mut self) -> Result<Vec<u8>>;
}

impl CommandExtensions for Command {
    fn pde_run(&mut self) -> Result<Vec<u8>> {
        let output = self.output()?;

        if output.status.success() {
            return Ok(output.stdout);
        }

        let err_out = String::from_utf8(output.stderr)?;

        let program = self.get_program().to_str().unwrap_or("");
        bail!("[{}]\n{}", program, &err_out)
    }
}
