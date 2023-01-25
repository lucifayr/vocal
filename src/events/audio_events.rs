use std::time::Instant;

use tui::backend::Backend;

use crate::instance::audio_instance::AudioInstance;

use super::handler::{Event, EventHandler};

pub enum AudioEvent {
    StartAudio,
    EndAudio,
    StartQueue,
    EndQueue,
    StopQueue,
    PauseAudio,
    MuteAudio,
    ResetSpeed,
    VolumeUp,
    VolumeDown,
    SpeedUp,
    SpeedDown,
    Tick,
    ResetTick,
}

trait AudioActions {
    fn start_queue(&mut self);
    fn pause(&mut self);
    fn mute(&mut self);
    fn reset_speed(&mut self);
    fn volume_up(&mut self);
    fn volume_down(&mut self);
    fn speed_up(&mut self);
    fn speed_down(&mut self);
    fn tick(&mut self);
    fn reset_tick(&mut self);
}

impl<B: Backend> AudioActions for EventHandler<B> {
    fn start_queue(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            AudioInstance::play_queue(instance.queue.clone(), self);
        }
    }

    fn pause(&mut self) {
        if let Some(instance) = self.audio_instance.as_mut() {
            instance.audio_options.is_paused = !instance.audio_options.is_paused;
            if instance.audio_options.is_paused {
                self.sink.pause();
            } else {
                self.sink.play();
            }
        }
    }

    fn mute(&mut self) {
        if !self.runtime_options.is_muted {
            self.sink.set_volume(0.0);
            self.runtime_options.is_muted = true;
        } else {
            self.sink.set_volume(self.runtime_options.volume_decimal);
            self.runtime_options.is_muted = false;
        }
    }

    fn reset_speed(&mut self) {
        self.runtime_options.speed = 100;
        self.runtime_options.speed_decimal = self.runtime_options.speed as f32 / 100.0;
        self.sink.set_speed(self.runtime_options.speed_decimal);
    }

    fn volume_up(&mut self) {
        self.runtime_options.volume /= 10;
        self.runtime_options.volume *= 10;

        if self.runtime_options.volume < 100 {
            self.runtime_options.volume += 10;
            self.runtime_options.volume_decimal = self.runtime_options.volume as f32 / 100.0;
            if !self.runtime_options.is_muted {
                self.sink.set_volume(self.runtime_options.volume_decimal);
            }
        }
    }

    fn volume_down(&mut self) {
        self.runtime_options.volume /= 10;
        self.runtime_options.volume *= 10;

        if self.runtime_options.volume > 0 {
            self.runtime_options.volume -= 10;
            self.runtime_options.volume_decimal = self.runtime_options.volume as f32 / 100.0;
            if !self.runtime_options.is_muted {
                self.sink.set_volume(self.runtime_options.volume_decimal);
            }
        }
    }

    fn speed_up(&mut self) {
        self.runtime_options.speed /= 10;
        self.runtime_options.speed *= 10;

        if self.runtime_options.speed < 200 {
            self.runtime_options.speed += 10;
            self.runtime_options.speed_decimal = self.runtime_options.speed as f32 / 100.0;
            self.sink.set_speed(self.runtime_options.speed_decimal);
        }
    }

    fn speed_down(&mut self) {
        self.runtime_options.speed /= 10;
        self.runtime_options.speed *= 10;

        if self.runtime_options.speed > 10 {
            self.runtime_options.speed -= 10;
            self.runtime_options.speed_decimal = self.runtime_options.speed as f32 / 100.0;
            self.sink.set_speed(self.runtime_options.speed_decimal);
        }
    }

    fn tick(&mut self) {
        if let Some(instance) = self.audio_instance.as_mut() {
            instance.audio_options.passed_time += instance
                .audio_options
                .time_since_last_tick
                .elapsed()
                .as_secs_f64()
                * self.runtime_options.speed_decimal as f64;
            instance.audio_options.time_since_last_tick = Instant::now();

            instance.audio_options.progress =
                instance.audio_options.passed_time / instance.audio_options.duration.as_secs_f64();
        }
    }

    fn reset_tick(&mut self) {
        if let Some(instance) = self.audio_instance.as_mut() {
            instance.audio_options.time_since_last_tick = Instant::now();
        }
    }
}

impl Event for AudioEvent {
    fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>) {
        match self {
            AudioEvent::StartAudio => {}
            AudioEvent::EndAudio => {}
            AudioEvent::StartQueue => handler.start_queue(),
            AudioEvent::EndQueue => {}
            AudioEvent::StopQueue => {}
            AudioEvent::PauseAudio => handler.pause(),
            AudioEvent::MuteAudio => handler.mute(),
            AudioEvent::ResetSpeed => handler.reset_speed(),
            AudioEvent::VolumeUp => handler.volume_up(),
            AudioEvent::VolumeDown => handler.volume_down(),
            AudioEvent::SpeedUp => handler.speed_up(),
            AudioEvent::SpeedDown => handler.speed_down(),
            AudioEvent::Tick => handler.tick(),
            AudioEvent::ResetTick => handler.reset_tick(),
        }
    }
}
