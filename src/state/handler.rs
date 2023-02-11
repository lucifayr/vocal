use tui::{backend::Backend, layout::Rect, Terminal};

use crate::input::config::Config;

use super::runtime_state::RuntimeState;

pub struct StateHandler<B: Backend> {
    pub state: RuntimeState,
    config: Config,
    pub terminal: Terminal<B>,
}

impl<B: Backend> StateHandler<B> {
    pub fn new(state: RuntimeState, config: Config, terminal: Terminal<B>) -> Self {
        StateHandler {
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
