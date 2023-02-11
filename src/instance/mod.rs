use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::{input::key::Key, state::handler::StateHandler};

pub mod player;
pub mod queue;
pub mod selection;

pub trait Instance {
    fn get_keybindings() -> Vec<Key>;
    fn process_input<B: Backend>(&mut self, handler: &mut StateHandler<B>, code: KeyCode);
}

pub trait InstanceRunable {
    fn run<B: Backend>(&mut self, handler: &mut StateHandler<B>);
}

pub trait InstanceRunableWithParent<I: Instance> {
    fn run<B: Backend>(&mut self, handler: &mut StateHandler<B>, parent: &mut I);
}
