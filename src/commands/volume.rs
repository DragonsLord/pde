use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{
    config::Config,
    modules::{notification::Notification, volume::VolumeControl},
    utils::ascii_utils::get_ascii_progress_bar,
};

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

pub struct VolumeCommandHandler {
    ctl: VolumeControl,
    notification_timeout: i32,
    icons_dir: PathBuf,
}

impl VolumeCommandHandler {
    pub fn create(config: &Config) -> Self {
        Self {
            ctl: VolumeControl::new(
                &config.volume.audio_sink,
                config.volume.step,
                config.volume.limit,
            ),
            notification_timeout: config
                .volume
                .notification_timeout_ms
                .unwrap_or(config.general.notification_timeout_ms),
            icons_dir: config.general.icons_dir(),
        }
    }

    pub fn handle(self, cmd: &VolumeCommand) -> Result<()> {
        match cmd.command {
            VolumeSubcommands::Increase => {
                self.ctl.increment()?;
                self.notify("plus")?;
            }
            VolumeSubcommands::Decrease => {
                self.ctl.decrement()?;
                self.notify("minus")?;
            }
            VolumeSubcommands::ToggleMute => {
                self.ctl.toggle_mute()?;
                // TODO: improve mute notification
                self.notify("off")?;
            }
        }

        Ok(())
    }

    fn notify(self, icon: &str) -> Result<()> {
        let volume_value = self.ctl.get()?;
        let volume_pct = volume_value * 100f32;
        let progress_bar = get_ascii_progress_bar(volume_value, self.ctl.get_limit());
        Notification::message(&format!("Volume ({:.0}%)", volume_pct))
            .body(progress_bar)
            .transient()
            .timeout(self.notification_timeout)
            .icon(&self.icons_dir.join(format!("volume-{}.svg", icon)))
            .sync_group("pde_volume")
            .send()?;

        Ok(())
    }
}
