use rodio::Sink;
use tui::backend::Backend;

use crate::{
    audio::player::Player,
    events::{handler::EventHandler, queue_events::QueueEvent},
};

use super::Instance;

pub struct Queue {
    pub queue: Vec<String>,
    pub sink: Sink,
    pub interupted: bool,
    pub looping: bool,
}

impl Instance for Queue {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B, Queue>) {
        handler.clear_terminal().unwrap();

        let mut looping = true;
        while looping {
            looping = self.looping;

            for path in self.queue.iter() {
                if self.interupted {
                    handler.trigger(QueueEvent::EndQueue);
                    return;
                }

                let player = Player::new(path).unwrap();
                player.play(&mut handler);
            }
        }

        handler.trigger(QueueEvent::EndQueue);
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
