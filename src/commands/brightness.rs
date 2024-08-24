use anyhow::Result;
use clap::{Args, Subcommand};

use crate::modules::{
    brightness::{BrightnessControl, BrightnessControlStep},
    notification::Notification,
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

pub struct BrightnessCommandConfig {
    pub step: i8,
    pub notification_timeout: i32,
    pub keyboard_device: String,
}

pub struct BrightnessCommandHandler {
    config: BrightnessCommandConfig,
}

impl BrightnessCommandHandler {
    pub fn create(config: BrightnessCommandConfig) -> Self {
        Self { config }
    }

    pub fn handle(self, cmd: &BrightnessCommand) -> Result<()> {
        match &cmd.command {
            BrightnessSubcommands::Increase => {
                let ctl = self.screen_ctl();
                ctl.increment()?;
                self.notify(&ctl)?;
            }
            BrightnessSubcommands::Decrease => {
                let ctl = self.screen_ctl();
                ctl.decrement()?;
                self.notify(&ctl)?;
            }
            BrightnessSubcommands::ToggleScreen => {
                let ctl = self.screen_ctl();
                ctl.toggle()?;
                self.notify(&ctl)?;
            }
            BrightnessSubcommands::KeyboardIncrease => {
                let ctl = self.keyboard_ctl();
                ctl.increment()?;
            }
            BrightnessSubcommands::KeyboardDecrease => {
                let ctl = self.keyboard_ctl();
                ctl.decrement()?;
            }
        }

        Ok(())
    }

    fn screen_ctl(&self) -> BrightnessControl {
        BrightnessControl::new(BrightnessControlStep::Percent(self.config.step), None)
    }

    fn keyboard_ctl(&self) -> BrightnessControl {
        BrightnessControl::new(
            BrightnessControlStep::Absolute(1),
            Some(self.config.keyboard_device.to_owned()),
        )
    }

    fn notify(self, ctl: &BrightnessControl) -> Result<()> {
        let brightness_value = ctl.get()?;
        Notification::message(&format!("Brightness: {:.0}%", brightness_value))
            .transient()
            .timeout(self.config.notification_timeout)
            .sync_group("pde_brightness")
            .send()?;

        Ok(())
    }
}
