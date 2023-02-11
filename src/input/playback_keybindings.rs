use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use tui::backend::Backend;

use crate::events::{
    handler::EventHandler, queue_events::QueueEvent, universal_events::UniversalEvent,
};

use super::key::Key;

pub struct PlaybackKeybindings {
    // universal
    pub quit: Key,

    // audio
    pub pause: Key,
    pub mute: Key,
    pub volume_up: Key,
    pub volume_down: Key,
    pub speed_up: Key,
    pub speed_down: Key,
    pub reset_speed: Key,

    // queue
    pub stop_queue: Key,
    pub loop_queue: Key,
    pub stop_loop_queue: Key,
}

impl std::default::Default for PlaybackKeybindings {
    fn default() -> Self {
        PlaybackKeybindings {
            quit: Key::new('Q', "Q", "quit"),
            pause: Key::new(' ', "Space", "pause"),
            mute: Key::new('m', "m", "mute"),
            volume_up: Key::new('k', "k", "volume up"),
            volume_down: Key::new('j', "j", "volume down"),
            speed_up: Key::new('L', "L", "speed up"),
            speed_down: Key::new('H', "H", "speed down"),
            reset_speed: Key::new('r', "r", "reset speed"),
            stop_queue: Key::new('q', "q", "stop queue and return to selection"),
            loop_queue: Key::new('l', "l", "loop queue"),
            stop_loop_queue: Key::new('L', "L", "stop looping queue"),
        }
    }
}

impl PlaybackKeybindings {
    pub fn get_keybindings(&self) -> Vec<&Key> {
        vec![
            &self.quit,
            &self.pause,
            &self.mute,
            &self.volume_up,
            &self.volume_down,
            &self.speed_up,
            &self.speed_down,
            &self.reset_speed,
            &self.stop_queue,
            &self.loop_queue,
            &self.stop_loop_queue,
        ]
    }

    // pub fn pull_input<B: Backend>(&self, handler: &mut EventHandler<B>) {
    //     if poll(Duration::from_millis(1)).unwrap_or(false) {
    //         if let Ok(Event::Key(key_event)) = read() {
    //             if key_event.code == KeyCode::Char(self.quit.key()) {
    //                 handler.trigger(UniversalEvent::QuitProgram)
    //             } else if key_event.code == KeyCode::Char(self.pause.key()) {
    //                 handler.trigger(AudioEvent::PauseAudio)
    //             } else if key_event.code == KeyCode::Char(self.mute.key()) {
    //                 handler.trigger(AudioEvent::MuteAudio)
    //             } else if key_event.code == KeyCode::Char(self.reset_speed.key()) {
    //                 handler.trigger(AudioEvent::ResetSpeed)
    //             } else if key_event.code == KeyCode::Char(self.volume_up.key()) {
    //                 handler.trigger(AudioEvent::VolumeUp)
    //             } else if key_event.code == KeyCode::Char(self.volume_down.key()) {
    //                 handler.trigger(AudioEvent::VolumeDown)
    //             } else if key_event.code == KeyCode::Char(self.speed_up.key()) {
    //                 handler.trigger(AudioEvent::SpeedUp)
    //             } else if key_event.code == KeyCode::Char(self.speed_down.key()) {
    //                 handler.trigger(AudioEvent::SpeedDown)
    //             } else if key_event.code == KeyCode::Char(self.stop_queue.key()) {
    //                 handler.trigger(QueueEvent::StopQueue)
    //             } else if key_event.code == KeyCode::Char(self.loop_queue.key()) {
    //                 handler.trigger(QueueEvent::LoopQueue)
    //             } else if key_event.code == KeyCode::Char(self.stop_loop_queue.key()) {
    //                 handler.trigger(QueueEvent::StopLoopQueue)
    //             }
    //         }
    //     }
    // }
}
