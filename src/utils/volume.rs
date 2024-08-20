use std::{
    ffi::OsStr,
    process::{Command, Output},
};

pub struct VolumeControl {
    sink: String,
    step_pct: i8,
    limit: f32,
}

impl VolumeControl {
    pub fn new(sink: &str, step_pct: i8, limit: f32) -> Self {
        Self {
            sink: sink.to_owned(),
            step_pct,
            limit,
        }
    }

    // TODO: proper error handling
    pub fn get(&self) -> f32 {
        let result = Self::exec("get-volume", [&self.sink]);
        let output = String::from_utf8(result.stdout).expect("valid get-volume output");

        output
            .split(' ')
            .skip(1)
            .take(1)
            .next()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap()
    }

    pub fn increment(&self) {
        self.set_volume(&format!("{}%+", self.step_pct));
    }

    pub fn decrement(&self) {
        self.set_volume(&format!("{}%-", self.step_pct));
    }

    pub fn set_volume(&self, volume_value: &str) {
        Self::exec(
            "set-volume",
            ["-l", &self.limit.to_string(), &self.sink, volume_value],
        );
    }

    pub fn toggle_mute(&self) {
        Self::exec("set-mute", [&self.sink, "toggle"]);
    }

    fn exec<Args, Arg>(cmd: &str, args: Args) -> Output
    // std::process::Child
    where
        Args: IntoIterator<Item = Arg>,
        Arg: AsRef<OsStr>,
    {
        Command::new("wpctl")
            .arg(cmd)
            .args(args)
            // .spawn()
            .output()
            .expect("wpctl failed")
    }
}
