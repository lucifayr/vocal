use tui::backend::Backend;

use crate::events::handler::EventHandler;

pub mod queue;
pub mod selection;

pub trait Instance {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B, Self>)
    where
        Self: Sized;
}
