use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use tui::backend::Backend;

use crate::{
    events::{
        handler::{trigger, EventHandler},
        queue_events::QueueEvent,
        universal_events::UniversalEvent,
    },
    instance::queue::Queue,
};

use super::key::Key;

const QUITE: Key = Key {
    key: KeyCode::Char('Q'),
    hint: "Q: quit",
};

const STOP: Key = Key {
    key: KeyCode::Char('q'),
    hint: "q: stop queue",
};

const LOOP: Key = Key {
    key: KeyCode::Char('l'),
    hint: "l: loop",
};

const STOP_LOOP: Key = Key {
    key: KeyCode::Char('L'),
    hint: "L: stop loop",
};

pub fn get_queue_keybindings() -> Vec<Key> {
    vec![QUITE, STOP, LOOP, STOP_LOOP]
}

pub fn poll_queue_input<B: Backend>(handler: &mut EventHandler<B>, instance: &mut Queue) {
    if poll(Duration::from_millis(1)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = read() {
            const QUIT_KEY: KeyCode = QUITE.key;
            const STOP_KEY: KeyCode = STOP.key;
            const LOOP_KEY: KeyCode = LOOP.key;
            const STOP_LOOP_KEY: KeyCode = STOP_LOOP.key;

            match key_event.code {
                QUIT_KEY => trigger(UniversalEvent::QuitProgram, handler, instance),
                STOP_KEY => trigger(QueueEvent::StopQueue, handler, instance),
                LOOP_KEY => trigger(QueueEvent::LoopQueue, handler, instance),
                STOP_LOOP_KEY => trigger(QueueEvent::StopLoopQueue, handler, instance),
                _ => {}
            }
        }
    }
}
