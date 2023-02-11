use rodio::Sink;
use tui::backend::Backend;

use crate::{
    audio::{player::Player, source_data::SourceData},
    events::{
        handler::{trigger, EventHandler},
        queue_events::QueueEvent,
    },
};

use super::Instance;

pub struct Queue {
    pub queue: Vec<String>,
    pub sink: Sink,
    pub interupted: bool,
    pub looping: bool,
}

impl Instance for Queue {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B>) {
        handler.clear_terminal().unwrap();

        let mut looping = true;
        while looping {
            looping = self.looping;

            for path in self.queue.clone().iter() {
                if self.interupted {
                    trigger(QueueEvent::EndQueue, handler, self);
                    return;
                }

                let mut player = Player::new(path).unwrap();
                player.play(SourceData::get_source(path).unwrap(), handler, self);
            }
        }

        trigger(QueueEvent::EndQueue, handler, self);
    }
}

impl Queue {
    pub fn new(queue: Vec<String>, sink: Sink) -> Self {
        Queue {
            queue,
            sink,
            interupted: false,
            looping: false,
        }
    }
}
