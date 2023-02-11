use tui::backend::Backend;

use crate::{
    audio::player::Player,
    events::{
        handler::{trigger, EventHandler},
        queue_events::QueueEvent,
    },
    input::{
        key::Key,
        queue_keybindings::{get_queue_keybindings, poll_queue_input},
    },
};

use super::Instance;

pub struct Queue {
    pub queue: Vec<String>,
    pub interupted: bool,
    pub looping: bool,
}

impl<I: Instance<()>> Instance<I> for Queue {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B>, parent: Option<I>) {
        handler.clear_terminal().unwrap();

        let mut looping = true;
        while looping {
            looping = self.looping;

            for path in self.queue.clone().iter() {
                if self.interupted {
                    trigger(QueueEvent::EndQueue, handler, self);
                    return;
                }

                let mut player = Player::new(
                    path,
                    handler.get_state().get_volume_decimal(),
                    handler.get_state().get_speed_decimal(),
                )
                .unwrap();
                player.run(handler, Some(self));
            }
        }

        trigger(QueueEvent::EndQueue, handler, self);
    }

    fn get_keybindings() -> Vec<Key> {
        get_queue_keybindings()
    }

    fn poll_input<B: Backend>(&mut self, handler: &mut EventHandler<B>) {
        poll_queue_input(handler, self)
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
