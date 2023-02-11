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
    fn stop_queue(instance: &mut Queue);
    fn loop_queue(instance: &mut Queue);
    fn stop_loop_queue(instance: &mut Queue);
}

impl<B: Backend> QueueActions for EventHandler<B> {
    fn stop_queue(instance: &mut Queue) {
        // instance.sink.stop();
        instance.interupted = true;
    }

    fn loop_queue(instance: &mut Queue) {
        instance.looping = true;
    }

    fn stop_loop_queue(instance: &mut Queue) {
        instance.looping = false;
    }
}

impl Event<Queue> for QueueEvent {
    fn trigger<B: Backend>(&self, _handler: &mut EventHandler<B>, instance: &mut Queue) {
        match self {
            QueueEvent::StartQueue => {}
            QueueEvent::EndQueue => {}
            QueueEvent::StopQueue => EventHandler::<B>::stop_queue(instance),
            QueueEvent::LoopQueue => EventHandler::<B>::loop_queue(instance),
            QueueEvent::StopLoopQueue => EventHandler::<B>::stop_loop_queue(instance),
        }
    }
}
