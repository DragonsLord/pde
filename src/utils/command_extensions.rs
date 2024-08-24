use anyhow::{bail, Result};
use std::process::Command;

pub trait CommandExtensions {
    fn pde_run(&mut self) -> Result<Vec<u8>>;

    fn killall_if_running(process: &str) -> Result<()>;

    fn dispatch(program: &str) -> Command;
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

    fn killall_if_running(process: &str) -> Result<()> {
        let out = Command::new("pgrep").arg(process).output()?;
        let exit_code = out
            .status
            .code()
            .expect("did not receive status code from pgrep");

        // found matched processes
        if exit_code == 0 {
            Command::new("killall").arg(process).pde_run()?;
        }

        Ok(())
    }

    // TODO: make configurable bin
    fn dispatch(program: &str) -> Command {
        let mut cmd = Command::new("hyprctl");
        cmd.args(["dispatch", "--", "exec", program]);
        cmd
    }
}
