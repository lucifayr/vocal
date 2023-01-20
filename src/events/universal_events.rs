use std::process::exit;

use crossterm::terminal::disable_raw_mode;
use tui::backend::Backend;

use super::handler::{Event, EventHandler};

pub enum UniversalEvent {
    QuitProgram,
}

trait UniversalActions {
    fn quit_program(&self);
}

impl<B: Backend> UniversalActions for EventHandler<B> {
    fn quit_program(&self) {
        disable_raw_mode().unwrap();
        exit(0)
    }
}

impl Event for UniversalEvent {
    fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>) {
        match self {
            UniversalEvent::QuitProgram => handler.quit_program(),
        }
    }
}
