use anyhow::{anyhow, Result};
use clap::Args;
use std::{
    io::{Cursor, Read},
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    config::profiles_config::{ModuleConfig, ProfilesConfig},
    utils::command_extensions::CommandExtensions,
};

#[derive(Args)]
pub struct InstallCommand {
    #[arg(short, long)]
    profile: Option<String>,
    profiles_path: Option<PathBuf>,
}

pub struct InstallCommandHandler {}

impl InstallCommandHandler {
    pub fn create() -> Self {
        Self {}
    }

    pub fn handle(self, cmd: &InstallCommand) -> Result<()> {
        let profile = self.resolve_profile(cmd)?;
        dbg!(&profile);

        for module in profile {
            module.execute()?;
        }

        Ok(())
    }

    fn resolve_profile(self, cmd: &InstallCommand) -> Result<Vec<ModuleConfig>> {
        let config = ProfilesConfig::parse(&cmd.profiles_path)?;
        dbg!(&config);

        let profile_name = cmd
            .profile.as_ref()
            .or(config.default_profile.as_ref())
            .ok_or(
                anyhow!("profile is not selected. Either provide it as cli arg or define default_profile field in profiles config")
            )?;

        let profile = config
            .profiles
            .get(profile_name)
            .ok_or(anyhow!("'{}' profile defintion not found", profile_name))?;

        profile
            .iter()
            .map(|module| {
                config
                    .modules
                    .get(module)
                    .ok_or(anyhow!("{} module not found", module))
                    .and_then(|cfg| match cfg {
                        // TODO: consider mapping to another model
                        ModuleConfig::Tool { tool, packages } => {
                            let resolved_tool = config
                                .tools
                                .get(tool)
                                .cloned()
                                .ok_or(anyhow!("{} tool not found", &tool))?;

                            Ok(ModuleConfig::Tool {
                                tool: resolved_tool,
                                packages: packages.clone(),
                            })
                        }
                        rest => Ok(rest.clone()),
                    })
            })
            .collect()
    }
}

impl ModuleConfig {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Tool { tool, packages } => Self::execute_tool(tool, packages),
            Self::Cmd { cmd } => Self::execute_cmd(cmd),
            Self::Zip {
                extract_zip_to,
                packages,
            } => Self::execute_zip(extract_zip_to, packages),
        }
    }

    fn execute_tool(tool: &str, packages: &Vec<String>) -> Result<()> {
        let mut cmd = Command::from_string(tool)?;
        for package in packages {
            cmd.arg(package);
        }
        cmd.pde_run()?;
        Ok(())
    }

    fn execute_cmd(commands: &Vec<String>) -> Result<()> {
        for cmd in commands {
            Command::from_string(cmd)?.pde_run()?;
        }
        Ok(())
    }

    fn execute_zip(target_dir: &Path, packages: &Vec<String>) -> Result<()> {
        for package_url in packages {
            let target_name = Self::package_name_from_path(package_url)?;
            let mut response = reqwest::blocking::get(package_url)?;
            dbg!(&response);
            let mut buffer = vec![];
            response.read_to_end(&mut buffer)?;

            let mut archive = zip::ZipArchive::new(Cursor::new(buffer))?;
            archive.extract(target_dir.join(target_name))?
        }

        Ok(())
    }

    fn package_name_from_path(url: &str) -> Result<String> {
        let name = url
            .split('/')
            .last()
            .ok_or(anyhow!("cannot retrieve package name from url: {}", url))?;

        Ok(name.replace(".zip", ""))
    }
}
