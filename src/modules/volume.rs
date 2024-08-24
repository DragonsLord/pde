use anyhow::{anyhow, Result};
use std::{ffi::OsStr, process::Command};

use crate::utils::command_extensions::CommandExtensions;

pub struct VolumeControl {
    sink: String,
    step_pct: i8,
    limit: f32,
}

impl VolumeControl {
    pub fn new(sink: &str, step_pct: i8, limit: f32) -> Self {
        Self {
            sink: sink.to_owned(),
            step_pct,
            limit,
        }
    }

    pub fn get(&self) -> Result<f32> {
        let stdout = Self::exec("get-volume", [&self.sink])?;
        let output = String::from_utf8(stdout)?;

        let value_str = output
            .split(' ')
            .skip(1)
            .take(1)
            .next()
            .ok_or(anyhow!("malformed get-volume output"))?;

        Ok(value_str.trim().parse()?)
    }

    pub fn increment(&self) -> Result<()> {
        self.set_volume(&format!("{}%+", self.step_pct))
    }

    pub fn decrement(&self) -> Result<()> {
        self.set_volume(&format!("{}%-", self.step_pct))
    }

    pub fn set_volume(&self, volume_value: &str) -> Result<()> {
        Self::exec(
            "set-volume",
            ["-l", &self.limit.to_string(), &self.sink, volume_value],
        )?;
        Ok(())
    }

    pub fn toggle_mute(&self) -> Result<()> {
        Self::exec("set-mute", [&self.sink, "toggle"])?;
        Ok(())
    }

    fn exec<Args, Arg>(cmd: &str, args: Args) -> Result<Vec<u8>>
    where
        Args: IntoIterator<Item = Arg>,
        Arg: AsRef<OsStr>,
    {
        Command::new("wpctl").arg(cmd).args(args).pde_run()
    }
}
