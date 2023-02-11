use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::{events::handler::EventHandler, input::key::Key};

pub mod queue;
pub mod selection;

pub trait Instance {
    fn get_keybindings() -> Vec<Key>;
    fn process_input<B: Backend>(&mut self, handler: &mut EventHandler<B>, code: KeyCode);
}

pub trait InstanceRunable {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B>);
}

pub trait InstanceRunableWithParent<I: Instance> {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B>, parent: &mut I);
}
