use anyhow::Result;
use clap::{Args, Subcommand};
use std::process::Command;

use crate::utils::command_extensions::CommandExtensions;

#[derive(Args)]
pub struct ApplicationCommand {
    #[command(subcommand)]
    command: ApplicationSubcommands,
}

#[derive(Subcommand)]
enum ApplicationSubcommands {
    Toggle {
        app: String,
        #[arg(short, long)]
        special_workspace: Option<String>,
    },
}

pub struct ApplicationCommandHandler {}

impl ApplicationCommandHandler {
    pub fn create() -> Self {
        Self {}
    }

    // TODO: notifications on failures, cursor progress?
    pub fn handle(self, cmd: &ApplicationCommand) -> Result<()> {
        match &cmd.command {
            ApplicationSubcommands::Toggle {
                app,
                special_workspace,
            } => {
                if !Command::is_running(app)? {
                    //TODO: app args support
                    Command::dispatch(app).pde_run()?;
                    return Ok(());
                }

                if let Some(workspace) = special_workspace {
                    Command::new("hyprctl")
                        .args(["dispatch", "togglespecialworkspace"])
                        .arg(workspace)
                        .pde_run()?;
                } else {
                    Command::new("killall").arg(app).pde_run()?;
                }
            }
        }

        Ok(())
    }
}
