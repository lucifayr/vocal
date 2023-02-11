use std::fmt::Display;

use log::info;
use tui::backend::Backend;

use crate::{instance::Instance, state::handler::StateHandler};

pub trait Event<I: Instance> {
    fn trigger<B: Backend>(&self, handler: &mut StateHandler<B>, instance: &mut I);
}

pub fn trigger<B: Backend, I: Instance, E: Event<I> + Display>(
    event: E,
    handler: &mut StateHandler<B>,
    instance: &mut I,
) {
    info!("{event}");
    event.trigger(handler, instance);
}
