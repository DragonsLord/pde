use anyhow::Result;
use clap::{Args, Subcommand};
use hyprland::{
    data::Clients,
    dispatch::{Dispatch, DispatchType},
    shared::HyprData,
};
use std::process::Command;

use crate::{modules::flatpak::Flatpak, utils::command_extensions::CommandExtensions};

#[derive(Args, Debug)]
pub struct ApplicationCommand {
    #[command(subcommand)]
    command: ApplicationSubcommands,
}

#[derive(Subcommand, Debug)]
enum ApplicationSubcommands {
    Toggle {
        app: String,
        #[arg(long)]
        flatpak: bool,
        #[arg(short, long)]
        special_workspace: Option<String>,
        #[arg(long)]
        window_rules: Vec<String>,
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
                window_rules,
            } => {
                if flatpak.clone() {
                    Self::handle_flatpak_app(app, special_workspace)?
                } else {
                    Self::handle_sys_app(app, special_workspace, window_rules)?
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

    fn handle_sys_app(
        app: &str,
        special_workspace: &Option<String>,
        window_rules: &Vec<String>,
    ) -> Result<()> {
        let running = Self::is_sys_app_running(app, special_workspace)?;

        if !running {
            let rules = window_rules
                .iter()
                .cloned()
                .chain(if let Some(workspace) = special_workspace {
                    vec![format!("workspace special:{}", workspace)]
                } else {
                    vec![]
                })
                .collect::<Vec<_>>()
                .join("; ");

            //TODO: app args support
            let exec_cmd = format!("[{}] {}", rules, app);
            Dispatch::call(DispatchType::Exec(&exec_cmd))?;
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

    fn is_sys_app_running(app: &str, special_workspace: &Option<String>) -> Result<bool> {
        let mut client_iter = Clients::get()?
            .into_iter()
            .filter(|c| c.initial_class == app);

        if let Some(workspace_name) = special_workspace {
            // For special workspaces check only hyprland clients
            return Ok(
                client_iter.any(|c| c.workspace.name == format!("special:{}", workspace_name))
            );
        }

        if client_iter.count() > 0 {
            return Ok(true);
        }

        // if no hyprland clients check sys processes
        Command::is_running(app)
    }
}
