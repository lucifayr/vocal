use std::fmt::Display;

use tui::backend::Backend;

use crate::{instance::queue::Queue, state::handler::StateHandler};

use super::event::Event;

pub enum QueueEvent {
    Stop,
    Loop,
    StopLoop,
    Next,
    Previous,
    AudioFinished,
}

impl Display for QueueEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            QueueEvent::Stop => "-QUEUE- Queue has stopped",
            QueueEvent::Loop => "-QUEUE- Looping enabled",
            QueueEvent::StopLoop => "-QUEUE- Looping disabled",
            QueueEvent::Next => "-QUEUE- Went to next audio",
            QueueEvent::Previous => "-QUEUE- Went to previous audio",
            QueueEvent::AudioFinished => "-QUEUE- An audio track has finished",
        };

        write!(f, "{msg}")
    }
}

trait QueueActions {
    fn stop_queue(&mut self, instance: &mut Queue);
    fn loop_queue(instance: &mut Queue);
    fn stop_loop_queue(instance: &mut Queue);
    fn next(instance: &mut Queue);
    fn prev(instance: &mut Queue);
    fn audio_finished(instance: &mut Queue);
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

    fn next(instance: &mut Queue) {
        instance.audio_changed = true;
        let index =
            (instance.current_audio_index as i64 + 1).clamp(0, instance.queue.len() as i64 - 1);
        instance.current_audio_index = index as usize;
    }

    fn prev(instance: &mut Queue) {
        instance.audio_changed = true;
        let index =
            (instance.current_audio_index as i64 - 1).clamp(0, instance.queue.len() as i64 - 1);
        instance.current_audio_index = index as usize;
    }

    fn audio_finished(instance: &mut Queue) {
        instance.current_audio_index += 1;
    }
}

impl Event<Queue> for QueueEvent {
    fn trigger<B: Backend>(&self, handler: &mut StateHandler<B>, instance: &mut Queue) {
        match self {
            QueueEvent::Stop => handler.stop_queue(instance),
            QueueEvent::Loop => StateHandler::<B>::loop_queue(instance),
            QueueEvent::StopLoop => StateHandler::<B>::stop_loop_queue(instance),
            QueueEvent::Next => StateHandler::<B>::next(instance),
            QueueEvent::Previous => StateHandler::<B>::prev(instance),
            QueueEvent::AudioFinished => StateHandler::<B>::audio_finished(instance),
        }
    }
}
