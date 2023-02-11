use std::fmt::Display;

use tui::backend::Backend;

use crate::{instance::queue::Queue, state::handler::StateHandler};

use super::event::Event;

#[allow(dead_code)]
pub enum QueueEvent {
    Start,
    Stop,
    Loop,
    StopLoop,
}

impl Display for QueueEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            QueueEvent::Start => "-QUEUE- Queue has started",
            QueueEvent::Stop => "-QUEUE- Queue has stopped",
            QueueEvent::Loop => "-QUEUE- looping enabled",
            QueueEvent::StopLoop => "-QUEUE- looping disabled",
        };

        write!(f, "{msg}")
    }
}

trait QueueActions {
    fn stop_queue(&mut self, instance: &mut Queue);
    fn loop_queue(instance: &mut Queue);
    fn stop_loop_queue(instance: &mut Queue);
}

impl<B: Backend> QueueActions for StateHandler<B> {
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
    fn trigger<B: Backend>(&self, handler: &mut StateHandler<B>, instance: &mut Queue) {
        match self {
            QueueEvent::Start => {}
            QueueEvent::Stop => handler.stop_queue(instance),
            QueueEvent::Loop => StateHandler::<B>::loop_queue(instance),
            QueueEvent::StopLoop => StateHandler::<B>::stop_loop_queue(instance),
        }
    }
}
