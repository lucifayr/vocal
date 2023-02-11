use tui::backend::Backend;

use crate::instance::queue::Queue;

use super::handler::{Event, EventHandler};

pub enum QueueEvent {
    StartQueue,
    EndQueue,
    StopQueue,
    LoopQueue,
    StopLoopQueue,
}

trait QueueActions {
    fn stop_queue(&mut self);
    fn loop_queue(&mut self);
    fn stop_loop_queue(&mut self);
}

impl<B: Backend> QueueActions for EventHandler<B, Queue> {
    fn stop_queue(&mut self) {
        self.instance.sink.stop();
        self.instance.interupted = true;
    }

    fn loop_queue(&mut self) {
        self.instance.looping = true;
    }

    fn stop_loop_queue(&mut self) {
        self.instance.looping = false;
    }
}

impl Event for QueueEvent {
    fn trigger_queue<B: Backend>(&self, handler: &mut EventHandler<B, Queue>) {
        match self {
            QueueEvent::StartQueue => {}
            QueueEvent::EndQueue => {}
            QueueEvent::StopQueue => handler.stop_queue(),
            QueueEvent::LoopQueue => handler.loop_queue(),
            QueueEvent::StopLoopQueue => handler.stop_loop_queue(),
        }
    }

    fn trigger_selection<B: Backend>(
        &self,
        handler: &mut EventHandler<B, crate::instance::selection::Selection>,
    ) {
    }
}
