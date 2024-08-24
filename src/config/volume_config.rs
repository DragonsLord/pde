use serde::Deserialize;

use super::defaults::Defaults;

#[derive(Deserialize, Debug)]
pub struct VolumeConfig {
    #[serde(default = "Defaults::audio_sink")]
    pub audio_sink: String,
    #[serde(default = "Defaults::control_step")]
    pub step: i8,
    #[serde(default = "Defaults::volume_limit")]
    pub limit: f32,
    #[serde(default)]
    pub notification_timeout_ms: Option<i32>,
}

impl Default for VolumeConfig {
    fn default() -> Self {
        Self {
            audio_sink: Defaults::audio_sink(),
            step: Defaults::control_step(),
            limit: Defaults::volume_limit(),
            notification_timeout_ms: None,
        }
    }
}
