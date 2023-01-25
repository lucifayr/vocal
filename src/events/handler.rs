use rodio::Sink;
use tui::{backend::Backend, Terminal};

use crate::{
    input::config::Config,
    instance::{
        audio_instance::AudioInstance, queue_instance::QueueInstance,
        selection_instace::SelectionInstance,
    },
    properties::runtime_properties::RuntimeOptions,
};

pub trait Event {
    fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>);
}

pub struct EventHandler<B: Backend> {
    pub runtime_options: RuntimeOptions,
    pub sink: Sink,
    pub config: Config,
    pub terminal: Terminal<B>,
    pub audio_instance: Option<AudioInstance>,
    pub selection_instance: Option<SelectionInstance>,
    pub queue_instance: Option<QueueInstance>,
}

impl<B: Backend> EventHandler<B> {
    pub fn new(
        sink: Sink,
        runtime_options: RuntimeOptions,
        config: Config,
        terminal: Terminal<B>,
    ) -> Self {
        EventHandler {
            sink,
            audio_instance: None,
            selection_instance: None,
            queue_instance: None,
            runtime_options,
            config,
            terminal,
        }
    }

    pub fn trigger<E: Event>(&mut self, event: E) {
        event.trigger(self);
    }
}
