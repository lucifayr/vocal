use tui::backend::Backend;

use crate::events::{audio_events::AudioEvent, handler::EventHandler};

use super::audio_instance::AudioInstance;

pub struct QueueInstance {
    queue: Vec<String>,
}

impl QueueInstance {
    pub fn new(queue: Vec<String>) -> Self {
        QueueInstance { queue }
    }

    pub fn play_queue<B: Backend>(handler: &mut EventHandler<B>) {
        match handler.terminal.clear() {
            Ok(_) => {}
            Err(_) => println!("Failed to clear terminal"),
        }

        if let Some(instance) = handler.queue_instance.as_mut() {
            for audio in instance.queue.clone().into_iter() {
                AudioInstance::start_instance(audio, handler)
            }

            handler.trigger(AudioEvent::EndQueue);
        }
    }
}
