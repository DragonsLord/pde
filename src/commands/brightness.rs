use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Args, Subcommand};

use crate::{
    config::Config,
    modules::{
        brightness::{BrightnessControl, BrightnessControlStep},
        notification::Notification,
    },
};

#[derive(Args)]
pub struct BrightnessCommand {
    #[command(subcommand)]
    command: BrightnessSubcommands,
}

#[derive(Subcommand)]
enum BrightnessSubcommands {
    #[clap(alias = "inc")]
    Increase,
    #[clap(alias = "dec")]
    Decrease,
    #[clap(alias = "-t")]
    ToggleScreen,
    #[clap(alias = "keyboard-inc")]
    KeyboardIncrease,
    #[clap(alias = "keyboard-dec")]
    KeyboardDecrease,
}

pub struct BrightnessCommandHandler {
    step: i8,
    notification_timeout: i32,
    keyboard_device: Option<String>,
    icons_dir: PathBuf,
}

impl BrightnessCommandHandler {
    pub fn create(config: &Config) -> Self {
        Self {
            step: config.brightness.step,
            notification_timeout: config
                .brightness
                .notification_timeout_ms
                .unwrap_or(config.general.notification_timeout_ms),
            keyboard_device: config.brightness.keyboard_device.to_owned(),
            icons_dir: config.general.icons_dir(),
        }
    }

    pub fn handle(self, cmd: &BrightnessCommand) -> Result<()> {
        match &cmd.command {
            BrightnessSubcommands::Increase => {
                let ctl = self.screen_ctl();
                ctl.increment()?;
                self.notify(&ctl, "plus")?;
            }
            BrightnessSubcommands::Decrease => {
                let ctl = self.screen_ctl();
                ctl.decrement()?;
                self.notify(&ctl, "minus")?;
            }
            BrightnessSubcommands::ToggleScreen => {
                let ctl = self.screen_ctl();
                ctl.toggle()?;
                self.notify(&ctl, "empty")?;
            }
            BrightnessSubcommands::KeyboardIncrease => {
                let ctl = self.keyboard_ctl()?;
                ctl.increment()?;
            }
            BrightnessSubcommands::KeyboardDecrease => {
                let ctl = self.keyboard_ctl()?;
                ctl.decrement()?;
            }
        }

        Ok(())
    }

    fn screen_ctl(&self) -> BrightnessControl {
        BrightnessControl::new(BrightnessControlStep::Percent(self.step), None)
    }

    fn keyboard_ctl(&self) -> Result<BrightnessControl> {
        if self.keyboard_device.is_none() {
            bail!("brightness.keyboard_device is not defined");
        }
        Ok(BrightnessControl::new(
            BrightnessControlStep::Absolute(1),
            self.keyboard_device.to_owned(),
        ))
    }

    fn notify(self, ctl: &BrightnessControl, icon: &str) -> Result<()> {
        let brightness_value = ctl.get()?;
        Notification::message(&format!("Brightness: {:.0}%", brightness_value))
            .transient()
            .timeout(self.notification_timeout)
            .sync_group("pde_brightness")
            .icon(&self.icons_dir.join(format!("brightness-{}.svg", icon)))
            .send()?;

        Ok(())
    }
}
