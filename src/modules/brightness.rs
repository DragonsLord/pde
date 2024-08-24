use anyhow::Result;
use std::{ffi::OsStr, process::Command};

use crate::utils::command_extensions::CommandExtensions;

pub enum BrightnessControlStep {
    Percent(i8),
    Absolute(i8),
}

pub struct BrightnessControl {
    device: Option<String>,
    step: BrightnessControlStep,
}

impl BrightnessControl {
    pub fn new(step: BrightnessControlStep, device: Option<String>) -> Self {
        Self { step, device }
    }

    pub fn get(&self) -> Result<i32> {
        let stdout = self.exec(["g", "-P"])?;
        let output = String::from_utf8(stdout)?;

        Ok(output.trim().parse()?)
    }

    pub fn increment(&self) -> Result<()> {
        let value = match self.step {
            BrightnessControlStep::Percent(pct) => format!("+{}%", pct),
            BrightnessControlStep::Absolute(val) => format!("{}+", val),
        };
        self.set_brightness(&value)
    }

    pub fn decrement(&self) -> Result<()> {
        let value = match self.step {
            BrightnessControlStep::Percent(pct) => format!("{}%-", pct),
            BrightnessControlStep::Absolute(val) => format!("{}-", val),
        };
        self.set_brightness(&value)
    }

    pub fn set_brightness(&self, value: &str) -> Result<()> {
        self.exec(["set", value])?;
        Ok(())
    }

    pub fn toggle(&self) -> Result<()> {
        if self.get()? == 0 {
            self.exec(["--restore"])?;
        } else {
            self.exec(["--save"])?;
            self.set_brightness("0")?;
        }
        Ok(())
    }

    fn exec<Args, Arg>(&self, args: Args) -> Result<Vec<u8>>
    where
        Args: IntoIterator<Item = Arg>,
        Arg: AsRef<OsStr>,
    {
        let mut prog = Command::new("brightnessctl");
        if let Some(device) = &self.device {
            prog.args(["-d", device]);
        }
        prog.args(args).pde_run()
    }
}
