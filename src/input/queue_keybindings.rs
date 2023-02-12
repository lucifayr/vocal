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
    key: KeyCode::Char('o'),
    hint: "o: loop",
};

const STOP_LOOP: Key = Key {
    key: KeyCode::Char('O'),
    hint: "O: stop loop",
};

const NEXT: Key = Key {
    key: KeyCode::Char('L'),
    hint: "L: next audio",
};

const PREV: Key = Key {
    key: KeyCode::Char('H'),
    hint: "H: previous audio",
};

pub fn get_queue_keybindings() -> Vec<Key> {
    vec![QUITE, STOP, LOOP, STOP_LOOP, NEXT, PREV]
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
    const NEXT_KEY: KeyCode = NEXT.key;
    const PREV_KEY: KeyCode = PREV.key;

    match code {
        QUIT_KEY => trigger(UniversalEvent::QuitProgram, handler, instance),
        STOP_KEY => trigger(QueueEvent::Stop, handler, instance),
        LOOP_KEY => trigger(QueueEvent::Loop, handler, instance),
        STOP_LOOP_KEY => trigger(QueueEvent::StopLoop, handler, instance),
        NEXT_KEY => trigger(QueueEvent::Next, handler, instance),
        PREV_KEY => trigger(QueueEvent::Previous, handler, instance),
        _ => {}
    }
}
