use super::command_extensions::CommandExtensions;
use anyhow::Result;
use std::process::Command;

pub struct Notification {
    message: String,
    icon: Option<String>,
    timeout: i32,
    urgency: String, // low, normal, critical
    transient: bool,
    hints: Vec<String>,
}

impl Notification {
    pub fn message(msg: &str) -> Self {
        Self {
            message: msg.to_owned(),
            icon: None,
            timeout: 3000, // TODO: default timeout from settings ??
            urgency: "low".to_owned(),
            transient: false,
            hints: vec![],
        }
    }

    pub fn icon(mut self, icon_path: &str) -> Self {
        self.icon = Some(icon_path.to_owned());
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
}

pub fn send_notification(notification: Notification) -> Result<()> {
    let mut cmd = Command::new("notify-send");

    if notification.transient {
        cmd.arg("-e");
    }

    if let Some(icon) = notification.icon {
        cmd.arg("-i ".to_owned() + &icon);
    }

    for hint in notification.hints {
        cmd.args(["-h", &hint]);
    }

    cmd.args(["-u", &notification.urgency])
        .args(["-t", &notification.timeout.to_string()])
        .arg(notification.message)
        .pde_run()?;

    Ok(())
}
