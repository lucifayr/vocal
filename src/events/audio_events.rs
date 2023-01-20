use super::handler::{Event, EventHandler};

pub enum AudioEvent {
    StartAudio,
    EndAudio,
    StartQueue,
    EndQueue,
    PauseAudio,
    MuteAudio,
    ResetSpeed,
    VolumeUp,
    VolumeDown,
    SpeedUp,
    SpeedDown,
}

trait AudioActions {
    fn pause(&mut self);
    fn mute(&mut self);
    fn reset_speed(&mut self);
    fn volume_up(&mut self);
    fn volume_down(&mut self);
    fn speed_up(&mut self);
    fn speed_down(&mut self);
}
impl AudioActions for EventHandler<'_> {
    fn pause(&mut self) {
        if let Some(audio_options) = &mut self.audio_options {
            audio_options.is_paused = !audio_options.is_paused;
            if audio_options.is_paused {
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
}

impl Event for AudioEvent {
    fn trigger(&self, handler: &mut EventHandler) {
        match self {
            AudioEvent::StartAudio => {}
            AudioEvent::EndAudio => {}
            AudioEvent::StartQueue => {}
            AudioEvent::EndQueue => {}
            AudioEvent::PauseAudio => handler.pause(),
            AudioEvent::MuteAudio => handler.mute(),
            AudioEvent::ResetSpeed => handler.reset_speed(),
            AudioEvent::VolumeUp => handler.volume_up(),
            AudioEvent::VolumeDown => handler.volume_down(),
            AudioEvent::SpeedUp => handler.speed_up(),
            AudioEvent::SpeedDown => handler.speed_down(),
        }
    }
}
