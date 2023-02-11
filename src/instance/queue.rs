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
    pub interupted: bool,
    pub looping: bool,
}

impl InstanceRunable for Queue {
    fn run<B: Backend>(&mut self, handler: &mut StateHandler<B>) {
        handler.clear_terminal().unwrap();

        let mut looping = true;
        while looping {
            for path in self.queue.clone().iter() {
                if self.interupted {
                    return;
                }

                let mut player = Player::new(
                    path,
                    handler.get_state().get_volume_decimal(),
                    handler.get_state().get_speed_decimal(),
                )
                .unwrap();

                player.run(handler, self);
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
        }
    }
}
