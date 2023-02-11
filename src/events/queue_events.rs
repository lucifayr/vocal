use tui::backend::Backend;

use crate::instance::queue::Queue;

use super::handler::{Event, EventHandler};

#[allow(dead_code)]
pub enum QueueEvent {
    Start,
    Stop,
    Loop,
    StopLoop,
}

trait QueueActions {
    fn stop_queue(&mut self, instance: &mut Queue);
    fn loop_queue(instance: &mut Queue);
    fn stop_loop_queue(instance: &mut Queue);
}

impl<B: Backend> QueueActions for EventHandler<B> {
    fn stop_queue(&mut self, instance: &mut Queue) {
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
    fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>, instance: &mut Queue) {
        match self {
            QueueEvent::Start => {}
            QueueEvent::Stop => handler.stop_queue(instance),
            QueueEvent::Loop => EventHandler::<B>::loop_queue(instance),
            QueueEvent::StopLoop => EventHandler::<B>::stop_loop_queue(instance),
        }
    }
}
