use anyhow::Result;
use std::{path::Path, process::Command};

use crate::{config::defaults::Defaults, utils::command_extensions::CommandExtensions};

pub struct Notification<'a> {
    message: String,
    icon: Option<&'a Path>,
    timeout: i32,
    urgency: String, // low, normal, critical
    transient: bool,
    hints: Vec<String>,
}

impl<'a> Notification<'a> {
    pub fn message(msg: &str) -> Self {
        Self {
            message: msg.to_owned(),
            icon: None,
            timeout: Defaults::notification_timeout_ms(),
            urgency: "low".to_owned(),
            transient: false,
            hints: vec![],
        }
    }

    pub fn icon(mut self, icon_path: &'a Path) -> Self {
        self.icon = Some(icon_path);
        self
    }

    pub fn timeout(mut self, timeout: i32) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn transient(mut self) -> Self {
        self.transient = true;
        self
    }

    pub fn sync_group(mut self, group: &str) -> Self {
        self.hints
            .push(format!("string:x-canonical-private-synchronous:{}", group));
        self
    }

    pub fn send(self) -> Result<()> {
        let mut cmd = Command::new("notify-send");

        if self.transient {
            cmd.arg("-e");
        }

        if let Some(icon) = self.icon {
            cmd.arg("-i").arg(icon);
        }

        for hint in self.hints {
            cmd.args(["-h", &hint]);
        }

        cmd.args(["-u", &self.urgency])
            .args(["-t", &self.timeout.to_string()])
            .arg(self.message)
            .pde_run()?;

        Ok(())
    }
}
