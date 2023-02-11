use std::fmt::Display;

use tui::backend::Backend;

use super::event::Event;
use crate::{instance::player::Player, state::handler::StateHandler};

pub enum PlayerEvent {
    Start,
    Stop,
    Pause,
    Mute,
    Unmute,
    VolumeUp,
    VolumeDown,
    SpeedUp,
    SpeedDown,
    ResetSpeed,
}

impl Display for PlayerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            PlayerEvent::Start => "-AUDIO TRACK- Audio has started",
            PlayerEvent::Stop => "-AUDIO TRACK- Audio has stopped",
            PlayerEvent::Pause => "-AUDIO TRACK- Audio has been paused",
            PlayerEvent::Mute => "-AUDIO TRACK- Audio has been muted",
            PlayerEvent::Unmute => "-AUDIO TRACK- Audio has been unmuted",
            PlayerEvent::VolumeUp => "-AUDIO TRACK- Volume has been increased",
            PlayerEvent::VolumeDown => "-AUDIO TRACK- Volume has been decreased",
            PlayerEvent::SpeedUp => "-AUDIO TRACK- Speed has been increased",
            PlayerEvent::SpeedDown => "-AUDIO TRACK- Speed has been decreased",
            PlayerEvent::ResetSpeed => "-AUDIO TRACK- Speed has been reset",
        };

        write!(f, "{msg}")
    }
}

trait PlayerActions {
    fn stop(instance: &mut Player);
    fn pause(instance: &mut Player);
    fn mute(&mut self, instance: &mut Player);
    fn unmute(&mut self, instance: &mut Player);
    fn volume_up(&mut self, instance: &mut Player);
    fn volume_down(&mut self, instance: &mut Player);
    fn speed_up(&mut self, instance: &mut Player);
    fn speed_down(&mut self, instance: &mut Player);
    fn reset_speed(&mut self, instance: &mut Player);
}

impl<B: Backend> PlayerActions for StateHandler<B> {
    fn stop(instance: &mut Player) {
        instance.sink.stop();
    }

    fn pause(instance: &mut Player) {
        instance.state.is_paused = !instance.state.is_paused;
        if instance.state.is_paused {
            instance.sink.pause();
        } else {
            instance.sink.play();
        }
    }

    fn mute(&mut self, instance: &mut Player) {
        if self.state.is_muted {
            return;
        }

        self.state.is_muted = true;
        instance.sink.set_volume(0.0);
    }

    fn unmute(&mut self, instance: &mut Player) {
        if !self.state.is_muted {
            return;
        }

        self.state.is_muted = false;
        let volume = self.get_state().get_volume_decimal();
        instance.sink.set_volume(volume);
    }

    fn volume_up(&mut self, instance: &mut Player) {
        self.state.volume /= 10;
        self.state.volume *= 10;

        if self.state.volume < 100 {
            self.state.volume += 10;
            if !self.state.is_muted {
                instance
                    .sink
                    .set_volume(self.get_state().get_volume_decimal());
            }
        }
    }

    fn volume_down(&mut self, instance: &mut Player) {
        self.state.volume /= 10;
        self.state.volume *= 10;

        if self.state.volume > 0 {
            self.state.volume -= 10;
            if !self.state.is_muted {
                instance
                    .sink
                    .set_volume(self.get_state().get_volume_decimal());
            }
        }
    }

    fn speed_up(&mut self, instance: &mut Player) {
        self.state.speed /= 10;
        self.state.speed *= 10;

        if self.state.speed < 200 {
            self.state.speed += 10;
            instance
                .sink
                .set_speed(self.get_state().get_speed_decimal());
        }
    }

    fn speed_down(&mut self, instance: &mut Player) {
        self.state.speed /= 10;
        self.state.speed *= 10;

        if self.state.speed > 10 {
            self.state.speed -= 10;
            instance.sink.set_speed(self.state.get_speed_decimal());
        }
    }

    fn reset_speed(&mut self, instance: &mut Player) {
        self.state.speed = 100;
        instance
            .sink
            .set_speed(self.get_state().get_speed_decimal());
    }
}

impl Event<Player> for PlayerEvent {
    fn trigger<B: Backend>(&self, handler: &mut StateHandler<B>, instance: &mut Player) {
        match self {
            PlayerEvent::Start => {}
            PlayerEvent::Stop => StateHandler::<B>::stop(instance),
            PlayerEvent::Pause => StateHandler::<B>::pause(instance),
            PlayerEvent::Mute => handler.mute(instance),
            PlayerEvent::Unmute => handler.unmute(instance),
            PlayerEvent::VolumeUp => handler.volume_up(instance),
            PlayerEvent::VolumeDown => handler.volume_down(instance),
            PlayerEvent::SpeedUp => handler.speed_up(instance),
            PlayerEvent::SpeedDown => handler.speed_down(instance),
            PlayerEvent::ResetSpeed => handler.reset_speed(instance),
        }
    }
}
