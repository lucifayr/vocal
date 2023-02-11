use tui::{backend::Backend, layout::Rect, Terminal};

use crate::{
    input::config::Config,
    instance::{queue::Queue, selection::Selection, Instance},
    state::runtime_state::RuntimeState,
};

pub trait Event {
    fn trigger_queue<B: Backend>(&self, handler: &mut EventHandler<B, Queue>);
    fn trigger_selection<B: Backend>(&self, handler: &mut EventHandler<B, Selection>);
}

pub struct EventHandler<B: Backend, I: Instance> {
    pub instance: I,
    pub state: RuntimeState,
    config: Config,
    pub terminal: Terminal<B>,
}

impl<B: Backend, I: Instance> EventHandler<B, I> {
    pub fn new(instance: I, state: RuntimeState, config: Config, terminal: Terminal<B>) -> Self {
        EventHandler {
            instance,
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

impl<B: Backend> EventHandler<B, Queue> {
    pub fn trigger<E: Event>(&mut self, event: E) {
        // do logging
        event.trigger_queue(self);
    }
}

impl<B: Backend> EventHandler<B, Selection> {
    pub fn trigger<E: Event>(&mut self, event: E) {
        // do logging
        event.trigger_selection(self);
    }
}
