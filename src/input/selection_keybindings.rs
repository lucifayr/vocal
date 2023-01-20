use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{
    events::{
        handler::EventHandler, selection_events::SelectionEvent, universal_events::UniversalEvent,
    },
    instance::audio_instance::AudioInstance,
};

use super::{config::Config, key::Key};

pub struct SelectionKeybindings {
    pub quit: Key,
    pub play: Key,
    pub up: Key,
    pub down: Key,
    pub go_to_top: Key,
    pub go_to_bottom: Key,
    pub add_to_bottom_of_queue: Key,
    pub add_to_top_of_queue: Key,
    pub remove_from_queue: Key,
}

impl std::default::Default for SelectionKeybindings {
    fn default() -> Self {
        SelectionKeybindings {
            quit: Key::new("q", "quit"),
            play: Key::new("p", "play"),
            up: Key::new("k", "up"),
            down: Key::new("j", "down"),
            go_to_top: Key::new("g", "go to top"),
            go_to_bottom: Key::new("G", "go to bottom"),
            add_to_bottom_of_queue: Key::new("l", "add to top of queue"),
            add_to_top_of_queue: Key::new("L", "add to bottom of queue"),
            remove_from_queue: Key::new("h", "remove from queue"),
        }
    }
}

impl SelectionKeybindings {
    pub fn get_keybindings(&self) -> Vec<Key> {
        vec![
            self.quit.clone(),
            self.play.clone(),
            self.up.clone(),
            self.down.clone(),
            self.go_to_top.clone(),
            self.go_to_bottom.clone(),
            self.add_to_bottom_of_queue.clone(),
            self.add_to_top_of_queue.clone(),
            self.remove_from_queue.clone(),
        ]
    }

    pub fn pull_input<B: Backend>(
        &self,
        config: &Config,
        terminal: &mut Terminal<B>,
        handler: &mut EventHandler,
    ) {
        if poll(Duration::from_millis(1)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = read() {
                match key_event.code {
                    KeyCode::Char('q') => handler.trigger(UniversalEvent::QuitProgram),
                    KeyCode::Char('p') => {
                        AudioInstance::play_queue(
                            handler
                                .selection_instance
                                .as_ref()
                                .expect("Selection instance should exist")
                                .queue
                                .clone(),
                            config,
                            terminal,
                            handler,
                        );
                    }
                    KeyCode::Up | KeyCode::Char('k') => handler.trigger(SelectionEvent::MoveUp),
                    KeyCode::Down | KeyCode::Char('j') => handler.trigger(SelectionEvent::MoveDown),
                    KeyCode::Char('g') => handler.trigger(SelectionEvent::MoveToTop),
                    KeyCode::Char('G') => handler.trigger(SelectionEvent::MoveToBottom),
                    KeyCode::Right | KeyCode::Char('l') => {
                        handler.trigger(SelectionEvent::AddToTopOfQueue)
                    }
                    KeyCode::Char('L') => handler.trigger(SelectionEvent::AddToBottomOfQueue),
                    KeyCode::Left | KeyCode::Char('h') => {
                        handler.trigger(SelectionEvent::RemoveFromQueue)
                    }
                    _ => {}
                }
            }
        }
    }
}
