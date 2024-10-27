use anyhow::{anyhow, bail, Result};
use std::process::Command;

pub trait CommandExtensions {
    fn pde_run(&mut self) -> Result<Vec<u8>>;

    fn is_running(process: &str) -> Result<bool>;

    fn killall_if_running(process: &str) -> Result<()>;

    fn dispatch(program: &str) -> Command;

    fn from_string(cmd_str: &str) -> Result<Command>;
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

    fn is_running(process: &str) -> Result<bool> {
        let out = Command::new("pgrep").arg(process).output()?;
        let exit_code = out
            .status
            .code()
            .expect("did not receive status code from pgrep");

        // found matched processes
        Ok(exit_code == 0)
    }

    fn killall_if_running(process: &str) -> Result<()> {
        if Self::is_running(process)? {
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

    fn from_string(cmd_str: &str) -> Result<Command> {
        let mut cmd_iter = cmd_str.split(" ");
        let cmd_name = cmd_iter
            .next()
            .ok_or(anyhow!("malformed '{}' command", cmd_str))?;

        let mut cmd = Command::new(cmd_name);
        for arg in cmd_iter {
            cmd.arg(arg);
        }

        Ok(cmd)
    }
}
