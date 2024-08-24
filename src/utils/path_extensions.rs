use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};

pub trait PathExtensions {
    fn pde_resolve(&self) -> Result<PathBuf>;
}

impl PathExtensions for PathBuf {
    fn pde_resolve(&self) -> Result<PathBuf> {
        let resolved_path = if self.starts_with("~") {
            let home_dir =
                env::var("HOME").map_err(|err| anyhow!("Failed to get $HOME: {}", err))?;
            &self
                .components()
                .skip(1)
                .fold(PathBuf::from(&home_dir), |mut agg, component| {
                    agg.push(component);
                    agg
                })
        } else {
            self
        };

        Ok(resolved_path.to_owned())
    }
}
