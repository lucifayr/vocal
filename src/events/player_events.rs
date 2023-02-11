use std::time::Instant;

use tui::backend::Backend;

use crate::{instance::player::Player, state::handler::StateHandler};

use super::event::Event;

pub enum PlayerEvent {
    Start,
    Stop,
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

trait PlayerActions {
    fn stop(instance: &mut Player);
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
        if !self.state.is_muted {
            self.state.is_muted = true;
            instance.sink.set_volume(0.0);
        } else {
            self.state.is_muted = false;
            instance
                .sink
                .set_volume(self.get_state().get_volume_decimal());
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

impl Event<Player> for PlayerEvent {
    fn trigger<B: Backend>(&self, handler: &mut StateHandler<B>, instance: &mut Player) {
        match self {
            PlayerEvent::Start => {}
            PlayerEvent::Stop => StateHandler::<B>::stop(instance),
            PlayerEvent::Pause => StateHandler::<B>::pause(instance),
            PlayerEvent::Mute => handler.mute(instance),
            PlayerEvent::ResetSpeed => handler.reset_speed(instance),
            PlayerEvent::VolumeUp => handler.volume_up(instance),
            PlayerEvent::VolumeDown => handler.volume_down(instance),
            PlayerEvent::SpeedUp => handler.speed_up(instance),
            PlayerEvent::SpeedDown => handler.speed_down(instance),
            PlayerEvent::Tick => {
                StateHandler::<B>::tick(instance, handler.get_state().get_speed_decimal() as f64)
            }
            PlayerEvent::ResetTick => StateHandler::<B>::reset_tick(instance),
        }
    }
}
