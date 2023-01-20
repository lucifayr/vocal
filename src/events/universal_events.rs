use std::process::exit;

use crossterm::terminal::disable_raw_mode;

use super::handler::{Event, EventHandler};

pub enum UniversalEvent {
    QuitProgram,
}

trait UniversalActions {
    fn quit_program(&self);
}

impl UniversalActions for EventHandler<'_> {
    fn quit_program(&self) {
        disable_raw_mode().unwrap();
        exit(0)
    }
}

impl Event for UniversalEvent {
    fn trigger(&self, handler: &mut EventHandler) {
        match self {
            UniversalEvent::QuitProgram => handler.quit_program(),
        }
    }
}
