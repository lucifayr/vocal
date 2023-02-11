use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::{
    events::{event::trigger, queue_events::QueueEvent, universal_events::UniversalEvent},
    instance::queue::Queue,
    state::handler::StateHandler,
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

pub fn process_queue_input<B: Backend>(
    handler: &mut StateHandler<B>,
    instance: &mut Queue,
    code: KeyCode,
) {
    const QUIT_KEY: KeyCode = QUITE.key;
    const STOP_KEY: KeyCode = STOP.key;
    const LOOP_KEY: KeyCode = LOOP.key;
    const STOP_LOOP_KEY: KeyCode = STOP_LOOP.key;

    match code {
        QUIT_KEY => trigger(UniversalEvent::QuitProgram, handler, instance),
        STOP_KEY => trigger(QueueEvent::Stop, handler, instance),
        LOOP_KEY => trigger(QueueEvent::Loop, handler, instance),
        STOP_LOOP_KEY => trigger(QueueEvent::StopLoop, handler, instance),
        _ => {}
    }
}
