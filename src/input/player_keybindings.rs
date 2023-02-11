use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::{
    events::{event::trigger, player_events::PlayerEvent},
    instance::player::Player,
    state::handler::StateHandler,
};

use super::key::Key;

const PAUSE: Key = Key {
    key: KeyCode::Char(' '),
    hint: "Space: pause",
};

const MUTE: Key = Key {
    key: KeyCode::Char('m'),
    hint: "m: mute",
};

const VOLUME_UP: Key = Key {
    key: KeyCode::Char('k'),
    hint: "k: volume up",
};

const VOLUME_DOWN: Key = Key {
    key: KeyCode::Char('j'),
    hint: "j: volume down",
};

const SPEED_UP: Key = Key {
    key: KeyCode::Char('K'),
    hint: "K: speed up",
};

const SPEED_DOWN: Key = Key {
    key: KeyCode::Char('J'),
    hint: "J: speed down",
};

const RESET_SPEED: Key = Key {
    key: KeyCode::Char('r'),
    hint: "r: reset speed",
};

pub fn get_player_keybindings() -> Vec<Key> {
    vec![
        PAUSE,
        MUTE,
        VOLUME_UP,
        VOLUME_DOWN,
        SPEED_UP,
        SPEED_DOWN,
        RESET_SPEED,
    ]
}

pub fn process_player_input<B: Backend>(
    handler: &mut StateHandler<B>,
    instance: &mut Player,
    code: KeyCode,
) {
    const PAUSE_KEY: KeyCode = PAUSE.key;
    const MUTE_KEY: KeyCode = MUTE.key;
    const VOLUME_UP_KEY: KeyCode = VOLUME_UP.key;
    const VOLUME_DOWN_KEY: KeyCode = VOLUME_DOWN.key;
    const SPEED_UP_KEY: KeyCode = SPEED_UP.key;
    const SPEED_DOWN_KEY: KeyCode = SPEED_DOWN.key;
    const RESET_SPEED_KEY: KeyCode = RESET_SPEED.key;

    match code {
        PAUSE_KEY => trigger(PlayerEvent::Pause, handler, instance),
        MUTE_KEY => trigger(PlayerEvent::Mute, handler, instance),
        VOLUME_UP_KEY => trigger(PlayerEvent::VolumeUp, handler, instance),
        VOLUME_DOWN_KEY => trigger(PlayerEvent::VolumeDown, handler, instance),
        SPEED_UP_KEY => trigger(PlayerEvent::SpeedUp, handler, instance),
        SPEED_DOWN_KEY => trigger(PlayerEvent::SpeedDown, handler, instance),
        RESET_SPEED_KEY => trigger(PlayerEvent::ResetSpeed, handler, instance),
        _ => {}
    }
}
