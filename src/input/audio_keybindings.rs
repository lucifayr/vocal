use std::{process::exit, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::disable_raw_mode,
};
use rodio::Sink;

use crate::properties::{audio_properties::AudioOptions, runtime_properties::RuntimeOptions};

use super::key::Key;

pub struct AudioKeybindings {
    pub quit: Key,
    pub pause: Key,
    pub mute: Key,
    pub volume_up: Key,
    pub volume_down: Key,
    pub speed_up: Key,
    pub speed_down: Key,
    pub reset_speed: Key,
}

impl std::default::Default for AudioKeybindings {
    fn default() -> Self {
        AudioKeybindings {
            quit: Key::new("Q", "quit"),
            pause: Key::new("Space", "pause"),
            mute: Key::new("m", "mute"),
            volume_up: Key::new("k", "volume up"),
            volume_down: Key::new("j", "volume down"),
            speed_up: Key::new("L", "speed up"),
            speed_down: Key::new("H", "speed down"),
            reset_speed: Key::new("r", "reset speed"),
        }
    }
}

impl AudioKeybindings {
    pub fn get_keybindings(&self) -> Vec<Key> {
        vec![
            self.quit.clone(),
            self.pause.clone(),
            self.mute.clone(),
            self.volume_up.clone(),
            self.volume_down.clone(),
            self.speed_up.clone(),
            self.speed_down.clone(),
            self.reset_speed.clone(),
        ]
    }

    pub fn pull_input(
        &self,
        sink: &Sink,
        runtime_options: &mut RuntimeOptions,
        audio_options: &mut AudioOptions,
    ) {
        if poll(Duration::from_millis(1)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = read() {
                match key_event.code {
                    KeyCode::Char('Q') => {
                        disable_raw_mode().unwrap();
                        exit(0);
                    }
                    KeyCode::Char(' ') => pause(sink, audio_options),
                    KeyCode::Char('m') => mute(sink, runtime_options),
                    KeyCode::Char('r') => reset_speed(sink, runtime_options),
                    KeyCode::Up | KeyCode::Char('k') => volume_up(sink, runtime_options),
                    KeyCode::Down | KeyCode::Char('j') => volume_down(sink, runtime_options),
                    KeyCode::Char('L') => speed_up(sink, runtime_options),
                    KeyCode::Char('H') => speed_down(sink, runtime_options),
                    _ => {}
                }
            }
        }
    }
}

fn pause(sink: &Sink, audio_options: &mut AudioOptions) {
    audio_options.is_paused = !audio_options.is_paused;
    if audio_options.is_paused {
        sink.pause();
    } else {
        sink.play();
    }
}

fn mute(sink: &Sink, runtime_options: &mut RuntimeOptions) {
    if !runtime_options.is_muted {
        sink.set_volume(0.0);
        runtime_options.is_muted = true;
    } else {
        sink.set_volume(runtime_options.volume_decimal);
        runtime_options.is_muted = false;
    }
}

fn reset_speed(sink: &Sink, runtime_options: &mut RuntimeOptions) {
    runtime_options.speed = 100;
    runtime_options.speed_decimal = runtime_options.speed as f32 / 100.0;
    sink.set_speed(runtime_options.speed_decimal);
}

fn volume_up(sink: &Sink, runtime_options: &mut RuntimeOptions) {
    runtime_options.volume /= 10;
    runtime_options.volume *= 10;

    if runtime_options.volume < 100 {
        runtime_options.volume += 10;
        runtime_options.volume_decimal = runtime_options.volume as f32 / 100.0;
        if !runtime_options.is_muted {
            sink.set_volume(runtime_options.volume_decimal);
        }
    }
}

fn volume_down(sink: &Sink, runtime_options: &mut RuntimeOptions) {
    runtime_options.volume /= 10;
    runtime_options.volume *= 10;

    if runtime_options.volume > 0 {
        runtime_options.volume -= 10;
        runtime_options.volume_decimal = runtime_options.volume as f32 / 100.0;
        if !runtime_options.is_muted {
            sink.set_volume(runtime_options.volume_decimal);
        }
    }
}

fn speed_up(sink: &Sink, runtime_options: &mut RuntimeOptions) {
    runtime_options.speed /= 10;
    runtime_options.speed *= 10;

    if runtime_options.speed < 200 {
        runtime_options.speed += 10;
        runtime_options.speed_decimal = runtime_options.speed as f32 / 100.0;
        sink.set_speed(runtime_options.speed_decimal);
    }
}

fn speed_down(sink: &Sink, runtime_options: &mut RuntimeOptions) {
    runtime_options.speed /= 10;
    runtime_options.speed *= 10;

    if runtime_options.speed > 10 {
        runtime_options.speed -= 10;
        runtime_options.speed_decimal = runtime_options.speed as f32 / 100.0;
        sink.set_speed(runtime_options.speed_decimal);
    }
}
