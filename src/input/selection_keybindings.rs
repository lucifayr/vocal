

use crossterm::event::{KeyCode};
use tui::backend::Backend;

use crate::{
    events::{
        handler::{trigger, EventHandler},
        selection_events::SelectionEvent,
        universal_events::UniversalEvent,
    },
    instance::selection::Selection,
};

use super::key::Key;

const QUIT: Key = Key {
    key: KeyCode::Char('q'),
    hint: "q: quit",
};

const PLAY: Key = Key {
    key: KeyCode::Char('p'),
    hint: "p: play",
};

const UP: Key = Key {
    key: KeyCode::Char('k'),
    hint: "k: up",
};

const DOWN: Key = Key {
    key: KeyCode::Char('j'),
    hint: "j: down",
};

const REMOVE: Key = Key {
    key: KeyCode::Char('h'),
    hint: "h: remove from queue",
};

const ADD_TO_TOP: Key = Key {
    key: KeyCode::Char('l'),
    hint: "l: add to top of queue",
};

const ADD_TO_BOTTOM: Key = Key {
    key: KeyCode::Char('L'),
    hint: "L: add to bottom of queue",
};

const GO_TO_TOP: Key = Key {
    key: KeyCode::Char('g'),
    hint: "g: go to top",
};

const GO_TO_BOTTOM: Key = Key {
    key: KeyCode::Char('G'),
    hint: "G: go to bottom",
};

pub fn get_selection_keybindings() -> Vec<Key> {
    vec![
        QUIT,
        PLAY,
        UP,
        DOWN,
        REMOVE,
        ADD_TO_TOP,
        ADD_TO_BOTTOM,
        GO_TO_TOP,
        GO_TO_BOTTOM,
    ]
}

pub fn process_selection_input<B: Backend>(
    handler: &mut EventHandler<B>,
    instance: &mut Selection,
    code: KeyCode,
) {
    const QUIT_KEY: KeyCode = QUIT.key;
    const PLAY_KEY: KeyCode = PLAY.key;
    const UP_KEY: KeyCode = UP.key;
    const DOWN_KEY: KeyCode = DOWN.key;
    const REMOVE_KEY: KeyCode = REMOVE.key;
    const ADD_TO_TOP_KEY: KeyCode = ADD_TO_TOP.key;
    const ADD_TO_BOTTOM_KEY: KeyCode = ADD_TO_BOTTOM.key;
    const GO_TO_TOP_KEY: KeyCode = GO_TO_TOP.key;
    const GO_TO_BOTTOM_KEY: KeyCode = GO_TO_BOTTOM.key;

    match code {
        QUIT_KEY => trigger(UniversalEvent::QuitProgram, handler, instance),
        PLAY_KEY => trigger(SelectionEvent::PlayQueue, handler, instance),
        UP_KEY => trigger(SelectionEvent::MoveUp, handler, instance),
        DOWN_KEY => trigger(SelectionEvent::MoveDown, handler, instance),
        REMOVE_KEY => trigger(SelectionEvent::RemoveFromQueue, handler, instance),
        ADD_TO_TOP_KEY => trigger(SelectionEvent::AddToTopOfQueue, handler, instance),
        ADD_TO_BOTTOM_KEY => trigger(SelectionEvent::AddToBottomOfQueue, handler, instance),
        GO_TO_TOP_KEY => trigger(SelectionEvent::MoveToTop, handler, instance),
        GO_TO_BOTTOM_KEY => trigger(SelectionEvent::MoveToBottom, handler, instance),
        _ => {}
    }
}
