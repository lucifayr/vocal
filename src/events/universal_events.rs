use std::{fmt::Display, process::exit};

use crossterm::terminal::disable_raw_mode;
use tui::backend::Backend;

use crate::{instance::Instance, state::handler::StateHandler};

use super::event::Event;

pub enum UniversalEvent {
    QuitProgram,
}

impl Display for UniversalEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            UniversalEvent::QuitProgram => "-UNIVERSAL- The program has been exited",
        };

        write!(f, "{msg}")
    }
}

trait UniversalActions {
    fn quit_program(&self);
}

impl<B: Backend> UniversalActions for StateHandler<B> {
    fn quit_program(&self) {
        disable_raw_mode().unwrap();
        exit(0)
    }
}

impl<I: Instance> Event<I> for UniversalEvent {
    fn trigger<B: Backend>(&self, handler: &mut StateHandler<B>, _instance: &mut I) {
        match self {
            UniversalEvent::QuitProgram => handler.quit_program(),
        }
    }
}
