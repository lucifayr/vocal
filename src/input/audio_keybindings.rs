use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use tui::backend::Backend;

use crate::events::{
    audio_events::AudioEvent, handler::EventHandler, universal_events::UniversalEvent,
};

use super::key::Key;

pub struct AudioKeybindings {
    pub quit: Key,
    pub stop_queue: Key,
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
            quit: Key::new('Q', "Q", "quit"),
            stop_queue: Key::new('q', "q", "stop queue and return to selection"),
            pause: Key::new(' ', "Space", "pause"),
            mute: Key::new('m', "m", "mute"),
            volume_up: Key::new('k', "k", "volume up"),
            volume_down: Key::new('j', "j", "volume down"),
            speed_up: Key::new('L', "L", "speed up"),
            speed_down: Key::new('H', "H", "speed down"),
            reset_speed: Key::new('r', "r", "reset speed"),
        }
    }
}

impl AudioKeybindings {
    pub fn get_keybindings(&self) -> Vec<&Key> {
        vec![
            &self.quit,
            &self.stop_queue,
            &self.pause,
            &self.mute,
            &self.volume_up,
            &self.volume_down,
            &self.speed_up,
            &self.speed_down,
            &self.reset_speed,
        ]
    }

    pub fn pull_input<B: Backend>(&self, handler: &mut EventHandler<B>) {
        if poll(Duration::from_millis(1)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = read() {
                if key_event.code == KeyCode::Char(self.quit.key()) {
                    handler.trigger(UniversalEvent::QuitProgram)
                } else if key_event.code == KeyCode::Char(self.stop_queue.key()) {
                    handler.trigger(AudioEvent::EndQueue)
                } else if key_event.code == KeyCode::Char(self.pause.key()) {
                    handler.trigger(AudioEvent::PauseAudio)
                } else if key_event.code == KeyCode::Char(self.mute.key()) {
                    handler.trigger(AudioEvent::MuteAudio)
                } else if key_event.code == KeyCode::Char(self.reset_speed.key()) {
                    handler.trigger(AudioEvent::ResetSpeed)
                } else if key_event.code == KeyCode::Char(self.volume_up.key()) {
                    handler.trigger(AudioEvent::VolumeUp)
                } else if key_event.code == KeyCode::Char(self.volume_down.key()) {
                    handler.trigger(AudioEvent::VolumeDown)
                } else if key_event.code == KeyCode::Char(self.speed_up.key()) {
                    handler.trigger(AudioEvent::SpeedUp)
                } else if key_event.code == KeyCode::Char(self.speed_down.key()) {
                    handler.trigger(AudioEvent::SpeedDown)
                }
            }
        }
    }
}
