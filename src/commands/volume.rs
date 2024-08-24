use anyhow::Result;
use clap::{Args, Subcommand};

use crate::modules::{notification::Notification, volume::VolumeControl};

#[derive(Args)]
pub struct VolumeCommand {
    #[command(subcommand)]
    command: VolumeSubcommands,
}

#[derive(Subcommand)]
enum VolumeSubcommands {
    #[clap(alias = "inc")]
    Increase,
    #[clap(alias = "dec")]
    Decrease,
    #[clap(alias = "-m")]
    ToggleMute,
}

pub struct VolumeCommandConfig {
    pub step: i8,
    pub limit: f32,
    pub notification_timeout: i32,
}

pub struct VolumeCommandHandler {
    ctl: VolumeControl,
    config: VolumeCommandConfig,
}

impl VolumeCommandHandler {
    pub fn create(config: VolumeCommandConfig) -> Self {
        Self {
            ctl: VolumeControl::new("@DEFAULT_AUDIO_SINK@", config.step, config.limit),
            config,
        }
    }

    pub fn handle(self, cmd: &VolumeCommand) -> Result<()> {
        match cmd.command {
            VolumeSubcommands::Increase => {
                self.ctl.increment()?;
                self.notify()?;
            }
            VolumeSubcommands::Decrease => {
                self.ctl.decrement()?;
                self.notify()?;
            }
            VolumeSubcommands::ToggleMute => {
                self.ctl.toggle_mute()?;
                self.notify()?;
            }
        }

        Ok(())
    }

    fn notify(self) -> Result<()> {
        let volume_value = self.ctl.get()?;
        let volume_pct = volume_value * 100f32;
        Notification::message(&format!("Volume: {:.0}%", volume_pct))
            .transient()
            .timeout(self.config.notification_timeout)
            .sync_group("pde_volume")
            .send()?;

        Ok(())
    }
}
