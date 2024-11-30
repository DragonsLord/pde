use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
//use hyprland::{data::Monitors, keyword::Keyword, shared::HyprData};

#[derive(Args)]
pub struct MonitorCommand {
    #[command(subcommand)]
    command: MonitorSubcommands,
}

#[derive(Subcommand)]
enum MonitorSubcommands {
    Toggle { monitor: String },
}

pub struct MonitorCommandHandler {}

impl MonitorCommandHandler {
    pub fn create() -> Self {
        Self {}
    }

    pub fn handle(self, cmd: &MonitorCommand) -> Result<()> {
        match &cmd.command {
            MonitorSubcommands::Toggle { monitor: _ } => {
                Err(anyhow!("work in progress"))
                //let monitors = Monitors::get()?;
                //let target_monitor = monitors
                //    .iter()
                //    .find(|m| m.name == *monitor)
                //    .ok_or(anyhow!("monitor not found"))?;
                //
                //if target_monitor.disabled {
                //    let monitor_cfg = format!(
                //        "{},{}x{}@{},{}x{}, {}",
                //        target_monitor.name,
                //        target_monitor.width,
                //        target_monitor.height,
                //        target_monitor.refresh_rate,
                //        target_monitor.x,
                //        target_monitor.y,
                //        target_monitor.scale
                //    );
                //    Keyword::set("monitor", monitor_cfg)?;
                //} else {
                //    Keyword::set("monitor", format!("{}, disable", target_monitor.name))?;
                //}
            }
        }
    }
}
