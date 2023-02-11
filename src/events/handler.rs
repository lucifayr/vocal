use tui::{backend::Backend, layout::Rect, Terminal};

use crate::{input::config::Config, instance::Instance, state::runtime_state::RuntimeState};

pub trait Event<I: Instance> {
    fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>, instance: &mut I);
}

pub struct EventHandler<B: Backend> {
    pub state: RuntimeState,
    config: Config,
    pub terminal: Terminal<B>,
}

impl<B: Backend> EventHandler<B> {
    pub fn new(state: RuntimeState, config: Config, terminal: Terminal<B>) -> Self {
        EventHandler {
            state,
            config,
            terminal,
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_state(&self) -> &RuntimeState {
        &self.state
    }

    pub fn clear_terminal(&mut self) -> Result<(), std::io::Error> {
        self.terminal.clear()
    }

    pub fn get_terminal_size(&self) -> Result<Rect, std::io::Error> {
        self.terminal.size()
    }
}

pub fn trigger<B: Backend, I: Instance, E: Event<I>>(
    event: E,
    handler: &mut EventHandler<B>,
    instance: &mut I,
) {
    // do logging
    event.trigger(handler, instance);
}
