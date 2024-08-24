use std::path::PathBuf;

use crate::utils::path_extensions::PathExtensions;

pub struct Defaults {}
impl Defaults {
    pub fn resource_root_dir() -> PathBuf {
        PathBuf::from("~/.desk-env/")
            .pde_resolve()
            .expect("default resource dir path invalid")
    }

    pub fn audio_sink() -> String {
        "@DEFAULT_AUDIO_SINK@".to_owned()
    }

    pub fn notification_timeout_ms() -> i32 {
        3000
    }

    pub fn control_step() -> i8 {
        2
    }

    pub fn volume_limit() -> f32 {
        1f32
    }
}
