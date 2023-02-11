use std::time::Instant;

use tui::backend::Backend;

use crate::audio::player::Player;

use super::handler::{Event, EventHandler};

pub enum AudioEvent {
    Start,
    End,
    Pause,
    Mute,
    ResetSpeed,
    VolumeUp,
    VolumeDown,
    SpeedUp,
    SpeedDown,
    Tick,
    ResetTick,
}

trait AudioActions {
    fn pause(instance: &mut Player);
    fn mute(&mut self, instance: &mut Player);
    fn reset_speed(&mut self, instance: &mut Player);
    fn volume_up(&mut self, instance: &mut Player);
    fn volume_down(&mut self, instance: &mut Player);
    fn speed_up(&mut self, instance: &mut Player);
    fn speed_down(&mut self, instance: &mut Player);
    fn tick(instance: &mut Player, speed: f64);
    fn reset_tick(instance: &mut Player);
}

impl<B: Backend> AudioActions for EventHandler<B> {
    fn pause(instance: &mut Player) {
        instance.state.is_paused = !instance.state.is_paused;
        if instance.state.is_paused {
            instance.sink.pause();
        } else {
            instance.sink.play();
        }
    }

    fn mute(&mut self, instance: &mut Player) {
        if !self.state.is_muted {
            instance.sink.set_volume(0.0);
            self.state.is_muted = true;
        } else {
            instance
                .sink
                .set_volume(self.get_state().get_volume_decimal());
            self.state.is_muted = false;
        }
    }

    fn reset_speed(&mut self, instance: &mut Player) {
        self.state.speed = 100;
        instance
            .sink
            .set_speed(self.get_state().get_speed_decimal());
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

    fn tick(instance: &mut Player, speed: f64) {
        instance.state.passed_time +=
            instance.state.time_since_last_tick.elapsed().as_secs_f64() * speed;
        instance.state.time_since_last_tick = Instant::now();

        instance.state.progress =
            instance.state.passed_time / instance.state.duration.as_secs_f64();
    }

    fn reset_tick(instance: &mut Player) {
        instance.state.time_since_last_tick = Instant::now();
    }
}

impl Event<Player> for AudioEvent {
    fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>, instance: &mut Player) {
        match self {
            AudioEvent::Start => {}
            AudioEvent::End => {}
            AudioEvent::Pause => EventHandler::<B>::pause(instance),
            AudioEvent::Mute => handler.mute(instance),
            AudioEvent::ResetSpeed => handler.reset_speed(instance),
            AudioEvent::VolumeUp => handler.volume_up(instance),
            AudioEvent::VolumeDown => handler.volume_down(instance),
            AudioEvent::SpeedUp => handler.speed_up(instance),
            AudioEvent::SpeedDown => handler.speed_down(instance),
            AudioEvent::Tick => {
                EventHandler::<B>::tick(instance, handler.get_state().get_speed_decimal() as f64)
            }
            AudioEvent::ResetTick => EventHandler::<B>::reset_tick(instance),
        }
    }
}
