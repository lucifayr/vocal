use tui::backend::Backend;

use crate::instance::queue_instance::QueueInstance;

use super::handler::{Event, EventHandler};

pub enum QueueEvent {
    StartQueue,
    EndQueue,
    StopQueue,
    LoopQueue,
    StopLoopQueue,
}

trait QueueActions {
    fn start_queue(&mut self);
    fn stop_queue(&mut self);
    fn loop_queue(&mut self);
    fn stop_loop_queue(&mut self);
}

impl<B: Backend> QueueActions for EventHandler<B> {
    fn start_queue(&mut self) {
        QueueInstance::play_queue(self)
    }

    fn stop_queue(&mut self) {
        if let Some(queue_instance) = self.queue_instance.as_mut() {
            if let Some(audio_instance) = self.audio_instance.as_mut() {
                self.sink.stop();
                queue_instance.interupted = true;
                audio_instance.interupted = true;
            }
        }
    }

    fn loop_queue(&mut self) {
        if let Some(instance) = self.queue_instance.as_mut() {
            instance.looping = true;
        }
    }

    fn stop_loop_queue(&mut self) {
        if let Some(instance) = self.queue_instance.as_mut() {
            instance.looping = false;
        }
    }
}

impl Event for QueueEvent {
    fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>) {
        match self {
            QueueEvent::StartQueue => handler.start_queue(),
            QueueEvent::EndQueue => {}
            QueueEvent::StopQueue => handler.stop_queue(),
            QueueEvent::LoopQueue => handler.loop_queue(),
            QueueEvent::StopLoopQueue => handler.stop_loop_queue(),
        }
    }
}
