use tui::backend::Backend;

use crate::{events::handler::EventHandler, input::key::Key};

pub mod queue;
pub mod selection;

pub trait Instance {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B>);
    fn get_keybindings() -> Vec<Key>;
    fn poll_input<B: Backend>(&mut self, handler: &mut EventHandler<B>);
}
