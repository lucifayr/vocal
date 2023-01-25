use tui::backend::Backend;

use crate::events::{handler::EventHandler, queue_events::QueueEvent};

use super::audio_instance::AudioInstance;

pub struct QueueInstance {
    queue: Vec<String>,
    pub interupted: bool,
    pub looping: bool,
}

impl QueueInstance {
    pub fn new(queue: Vec<String>) -> Self {
        QueueInstance {
            queue,
            interupted: false,
            looping: false,
        }
    }

    pub fn play_queue<B: Backend>(handler: &mut EventHandler<B>) {
        match handler.terminal.clear() {
            Ok(_) => {}
            Err(_) => println!("Failed to clear terminal"),
        }

        // ====================================================================================================
        // this is stupid please fix it
        //     =================================================================================================
        if handler.queue_instance.is_none() {
            handler.trigger(QueueEvent::EndQueue);
            return;
        }

        let mut looping = true;
        while looping {
            looping = handler.queue_instance.as_ref().unwrap().looping;

            for audio in handler
                .queue_instance
                .as_ref()
                .unwrap()
                .queue
                .clone()
                .into_iter()
            {
                if handler.queue_instance.as_ref().unwrap().interupted {
                    handler.trigger(QueueEvent::EndQueue);
                    return;
                }

                AudioInstance::start_instance(audio, handler);
            }
        }

        handler.trigger(QueueEvent::EndQueue);
    }
}
