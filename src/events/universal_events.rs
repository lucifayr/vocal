use std::process::exit;

use crossterm::terminal::disable_raw_mode;
use tui::backend::Backend;

use crate::instance::{queue::Queue, selection::Selection, Instance};

use super::handler::{Event, EventHandler};

pub enum UniversalEvent {
    QuitProgram,
}

trait UniversalActions {
    fn quit_program(&self);
}

impl<B: Backend, I: Instance> UniversalActions for EventHandler<B, I> {
    fn quit_program(&self) {
        disable_raw_mode().unwrap();
        exit(0)
    }
}

impl Event for UniversalEvent {
    fn trigger_queue<B: Backend>(&self, handler: &mut EventHandler<B, Queue>) {
        match self {
            UniversalEvent::QuitProgram => handler.quit_program(),
        }
    }
    fn trigger_selection<B: Backend>(&self, handler: &mut EventHandler<B, Selection>) {
        match self {
            UniversalEvent::QuitProgram => handler.quit_program(),
        }
    }
}
