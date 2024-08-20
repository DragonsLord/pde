use clap::Args;

use crate::utils::{
    notification::{send_notification, Notification},
    volume::VolumeControl,
};

#[derive(Args)]
pub struct VolumeCommand {
    #[command(flatten)]
    options: VolumeOptions,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct VolumeOptions {
    #[arg(short, long)]
    increment: bool,

    #[arg(short, long)]
    decrement: bool,

    #[arg(short = 'm', long)]
    toggle_mute: bool,
}

struct VolumeCommandHandler {
    ctl: VolumeControl,
}

impl VolumeCommandHandler {
    fn create() -> Self {
        Self {
            ctl: VolumeControl::new("@DEFAULT_AUDIO_SINK@", 2, 1.2),
        }
    }

    fn handle(self, cmd: &VolumeCommand) {
        if cmd.options.increment {
            self.ctl.increment();
            self.notify()
        } else if cmd.options.decrement {
            self.ctl.decrement();
            self.notify()
        } else if cmd.options.toggle_mute {
            self.ctl.toggle_mute();
            self.notify()
        }
    }

    fn notify(self) {
        let volume_value = self.ctl.get();
        let volume_pct = volume_value * 100f32;
        send_notification(
            Notification::message(&format!("Volume: {}%", volume_pct))
                .transient()
                .sync_group("pde_volume"),
        )
    }
}

pub fn execute_volume_command(cmd: &VolumeCommand) {
    VolumeCommandHandler::create().handle(cmd);
}
