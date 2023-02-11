// impl std::default::Default for PlaybackKeybindings {
//     fn default() -> Self {
//         PlaybackKeybindings {
//             quit: Key::new('Q', "Q", "quit"),
//             pause: Key::new(' ', "Space", "pause"),
//             mute: Key::new('m', "m", "mute"),
//             volume_up: Key::new('k', "k", "volume up"),
//             volume_down: Key::new('j', "j", "volume down"),
//             speed_up: Key::new('L', "L", "speed up"),
//             speed_down: Key::new('H', "H", "speed down"),
//             reset_speed: Key::new('r', "r", "reset speed"),
//             stop_queue: Key::new('q', "q", "stop queue and return to selection"),
//             loop_queue: Key::new('l', "l", "loop queue"),
//             stop_loop_queue: Key::new('L', "L", "stop looping queue"),
//         }
//     }
// }



use crossterm::event::{KeyCode};
use tui::backend::Backend;

use crate::{
    audio::player::Player,
    events::{
        audio_events::AudioEvent,
        handler::{trigger, EventHandler},
    },
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
    handler: &mut EventHandler<B>,
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
        PAUSE_KEY => trigger(AudioEvent::PauseAudio, handler, instance),
        MUTE_KEY => trigger(AudioEvent::MuteAudio, handler, instance),
        VOLUME_UP_KEY => trigger(AudioEvent::VolumeUp, handler, instance),
        VOLUME_DOWN_KEY => trigger(AudioEvent::VolumeDown, handler, instance),
        SPEED_UP_KEY => trigger(AudioEvent::SpeedUp, handler, instance),
        SPEED_DOWN_KEY => trigger(AudioEvent::SpeedDown, handler, instance),
        RESET_SPEED_KEY => trigger(AudioEvent::ResetSpeed, handler, instance),
        _ => {}
    }
}
