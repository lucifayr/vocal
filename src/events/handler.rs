use rodio::Sink;

use crate::properties::{audio_properties::AudioOptions, runtime_properties::RuntimeOptions};

pub trait Event {
    fn trigger(&self, handler: &mut EventHandler);
}

pub struct EventHandler<'a> {
    pub sink: &'a Sink,
    pub audio_options: Option<AudioOptions>,
    pub runtime_options: RuntimeOptions,
}

impl<'a> EventHandler<'a> {
    pub fn new(sink: &'a Sink, runtime_options: RuntimeOptions) -> Self {
        EventHandler {
            sink,
            audio_options: None,
            runtime_options,
        }
    }

    pub fn trigger<E: Event>(&mut self, event: E) {
        event.trigger(self);
    }
}
