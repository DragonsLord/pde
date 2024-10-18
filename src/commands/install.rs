use anyhow::{anyhow, Result};
use clap::Args;
use std::{
    io::{Cursor, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{
    config::profiles::{
        module_config::{ModuleConfig, ModuleStep},
        profiles_config::ProfilesConfig,
        tools_config::ToolConfig,
    },
    utils::command_extensions::CommandExtensions,
};

#[derive(Args)]
pub struct InstallCommand {
    #[arg(short, long)]
    profile: Option<String>,
    profiles_path: Option<PathBuf>,
}

#[derive(Debug)]
enum ProfileModule {
    Tool {
        name: String,
        config: ToolConfig,
        packages: Vec<String>,
    },
    Script {
        name: String,
        script: String,
    },
    Zip {
        name: String,
        target_dir: PathBuf,
        packages: Vec<String>,
    },
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

    fn resolve_profile(self, cmd: &InstallCommand) -> Result<Vec<ProfileModule>> {
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

        let modules: Result<Vec<&ModuleConfig>> = profile
            .iter()
            .map(|module| {
                config
                    .modules
                    .get(module)
                    .ok_or(anyhow!("{} module not found", module))
            })
            .collect();

        let resolved: Result<Vec<ProfileModule>, _> = modules?
            .into_iter()
            .flat_map(|cfg| cfg.steps.clone())
            .map(|cfg| match cfg {
                ModuleStep::Tool {
                    tool,
                    packages,
                    name,
                } => config
                    .tools
                    .get(&tool)
                    .cloned()
                    .ok_or(anyhow!("{} tool not found", &tool))
                    .map(|tool_config| ProfileModule::Tool {
                        name,
                        config: tool_config,
                        packages: packages.clone(),
                    }),
                ModuleStep::Script { script, name } => Ok(ProfileModule::Script { name, script }),
                ModuleStep::Zip {
                    name,
                    extract_zip_to,
                    packages,
                } => Ok(ProfileModule::Zip {
                    name,
                    target_dir: extract_zip_to.clone(),
                    packages: packages.clone(),
                }),
            })
            .collect();

        resolved
    }
}

impl ProfileModule {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Tool {
                name,
                config,
                packages,
            } => Self::execute_tool(name, config, packages),
            Self::Script { name, script } => Self::execute_script(name, &script),
            Self::Zip {
                name,
                target_dir,
                packages,
            } => Self::execute_zip(name, target_dir, packages),
        }
    }

    fn execute_tool(name: &str, tool: &ToolConfig, packages: &Vec<String>) -> Result<()> {
        println!("[{}] executing tool: {}", name, &tool.cmd);
        if tool.batching {
            let mut cmd = Command::from_string(&tool.cmd)?;
            for package in packages {
                cmd.arg(package);
            }
            cmd.stdout(Stdio::inherit()).pde_run()?;
            Ok(())
        } else {
            for package in packages {
                Command::from_string(&tool.cmd)?
                    .arg(package)
                    .stdout(Stdio::inherit())
                    .pde_run()?;
            }

            Ok(())
        }
    }

    fn execute_script(name: &str, script: &str) -> Result<()> {
        println!("[{}] runnning script", name);
        Command::new("sh")
            .arg("-c")
            .arg(script)
            .stdout(Stdio::inherit())
            .pde_run()?;
        Ok(())
    }

    fn execute_zip(name: &str, target_dir: &Path, packages: &Vec<String>) -> Result<()> {
        println!("[{}] getting zip package", name);
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
