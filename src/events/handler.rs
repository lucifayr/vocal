use rodio::Sink;

use crate::{
    instance::{audio_instance::AudioInstance, selection_instace::SelectionInstance},
    properties::runtime_properties::RuntimeOptions,
};

pub trait Event {
    fn trigger(&self, handler: &mut EventHandler);
}

pub struct EventHandler<'a> {
    pub runtime_options: RuntimeOptions,
    pub sink: &'a Sink,
    pub audio_instance: Option<AudioInstance>,
    pub selection_instance: Option<SelectionInstance>,
}

impl<'a> EventHandler<'a> {
    pub fn new(sink: &'a Sink, runtime_options: RuntimeOptions) -> Self {
        EventHandler {
            sink,
            audio_instance: None,
            selection_instance: None,
            runtime_options,
        }
    }

    pub fn trigger<E: Event>(&mut self, event: E) {
        event.trigger(self);
    }
}
