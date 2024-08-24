use serde::Deserialize;

use super::defaults::Defaults;

#[derive(Deserialize, Debug)]
pub struct BrightnessConfig {
    #[serde(default = "Defaults::control_step")]
    pub step: i8,
    pub keyboard_device: Option<String>,
    #[serde(default)]
    pub notification_timeout_ms: Option<i32>,
}

impl Default for BrightnessConfig {
    fn default() -> Self {
        Self {
            step: Defaults::control_step(),
            keyboard_device: None,
            notification_timeout_ms: None,
        }
    }
}
