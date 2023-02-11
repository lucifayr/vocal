use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

#[derive(Debug, Clone)]
pub struct Key {
    pub key: KeyCode,
    pub hint: &'static str,
}

pub fn poll_key() -> Option<KeyCode> {
    if poll(Duration::from_millis(1)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = read() {
            return Some(key_event.code);
        }
    }
    None
}
