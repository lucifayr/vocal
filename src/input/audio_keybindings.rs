use std::{process::exit, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::disable_raw_mode,
};

use crate::events::{audio_events::AudioEvent, handler::EventHandler};

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

    pub fn pull_input(&self, handler: &mut EventHandler) {
        if poll(Duration::from_millis(1)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = read() {
                match key_event.code {
                    KeyCode::Char('Q') => {
                        disable_raw_mode().unwrap();
                        exit(0);
                    }
                    KeyCode::Char(' ') => handler.trigger(AudioEvent::PauseAudio),
                    KeyCode::Char('m') => handler.trigger(AudioEvent::MuteAudio),
                    KeyCode::Char('r') => handler.trigger(AudioEvent::ResetSpeed),
                    KeyCode::Up | KeyCode::Char('k') => handler.trigger(AudioEvent::VolumeUp),
                    KeyCode::Down | KeyCode::Char('j') => handler.trigger(AudioEvent::VolumeDown),
                    KeyCode::Char('L') => handler.trigger(AudioEvent::SpeedUp),
                    KeyCode::Char('H') => handler.trigger(AudioEvent::SpeedDown),
                    _ => {}
                }
            }
        }
    }
}
