use anyhow::Result;
use clap::{Args, Subcommand};
use hyprland::dispatch::{Dispatch, DispatchType};
use std::process::Command;

use crate::{modules::flatpak::Flatpak, utils::command_extensions::CommandExtensions};

#[derive(Args)]
pub struct ApplicationCommand {
    #[command(subcommand)]
    command: ApplicationSubcommands,
}

#[derive(Subcommand)]
enum ApplicationSubcommands {
    Toggle {
        app: String,
        #[arg(long, default_value = "false")]
        flatpak: bool,
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
                flatpak,
                special_workspace,
            } => {
                if *flatpak {
                    Self::handle_flatpak_app(app, special_workspace)?
                } else {
                    Self::handle_sys_appa(app, special_workspace)?
                }
            }
        }

        Ok(())
    }

    fn handle_flatpak_app(app: &str, special_workspace: &Option<String>) -> Result<()> {
        if !Flatpak::is_running(app)? {
            Dispatch::call(DispatchType::Exec(&format!("flatpak run {}", app)))?;
            return Ok(());
        }

        if let Some(workspace) = special_workspace {
            Dispatch::call(DispatchType::ToggleSpecialWorkspace(Some(
                workspace.to_owned(),
            )))?
        } else {
            Flatpak::kill(app)?;
        }

        Ok(())
    }

    fn handle_sys_appa(app: &str, special_workspace: &Option<String>) -> Result<()> {
        if !Command::is_running(app)? {
            //TODO: app args support
            Dispatch::call(DispatchType::Exec(app))?;
            return Ok(());
        }

        if let Some(workspace) = special_workspace {
            Dispatch::call(DispatchType::ToggleSpecialWorkspace(Some(
                workspace.to_owned(),
            )))?
        } else {
            Command::new("killall").arg(app).pde_run()?;
        }

        Ok(())
    }
}
