use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::{
    events::{event::trigger, queue_events::QueueEvent},
    input::{
        key::Key,
        queue_keybindings::{get_queue_keybindings, process_queue_input},
    },
    state::handler::StateHandler,
};

use super::{player::Player, Instance, InstanceRunable, InstanceRunableWithParent};

pub struct Queue {
    pub queue: Vec<String>,
    pub current_audio_index: usize,
    pub audio_changed: bool,
    pub interupted: bool,
    pub looping: bool,
}

impl InstanceRunable for Queue {
    fn run<B: Backend>(&mut self, handler: &mut StateHandler<B>) {
        match handler.clear_terminal() {
            Ok(_) => {}
            Err(err) => log::error!("-ERROR- Filed to clear terminal: {err}"),
        }

        let mut looping = true;
        while looping {
            while self.current_audio_index < self.queue.len() {
                if self.interupted {
                    return;
                }

                self.current_audio_index = self.current_audio_index.clamp(0, self.queue.len() - 1);
                let path = self
                    .queue
                    .get(self.current_audio_index)
                    .expect("should exist");

                self.audio_changed = false;

                match Player::new(
                    path,
                    handler.get_state().get_volume_decimal(),
                    handler.get_state().get_speed_decimal(),
                ) {
                    Some(mut player) => player.run(handler, self),
                    None => {
                        log::error!("-ERROR- Failed to start audio player")
                    }
                }
            }

            looping = self.looping;
        }

        trigger(QueueEvent::Stop, handler, self);
    }
}

impl Instance for Queue {
    fn get_keybindings() -> Vec<Key> {
        get_queue_keybindings()
    }

    fn process_input<B: Backend>(&mut self, handler: &mut StateHandler<B>, code: KeyCode) {
        process_queue_input(handler, self, code);
    }
}

impl Queue {
    pub fn new(queue: Vec<String>) -> Self {
        Queue {
            queue,
            interupted: false,
            looping: false,
            audio_changed: false,
            current_audio_index: 0,
        }
    }
}
