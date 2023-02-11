use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use tui::backend::Backend;

use crate::{
    events::{
        handler::{trigger, EventHandler},
        selection_events::SelectionEvent,
        universal_events::UniversalEvent,
    },
    instance::selection::Selection,
};

use super::key::Key;

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
            quit: Key::new('q', "q", "pause"),
            play: Key::new('p', "p", "play"),
            up: Key::new('k', "k", "up"),
            down: Key::new('j', "j", "down"),
            go_to_top: Key::new('g', "g", "go to top"),
            go_to_bottom: Key::new('G', "G", "go to bottom"),
            add_to_top_of_queue: Key::new('l', "l", "add to top of queue"),
            add_to_bottom_of_queue: Key::new('L', "L", "add to bottom of queue"),
            remove_from_queue: Key::new('h', "h", "remove from queue"),
        }
    }
}

impl SelectionKeybindings {
    pub fn get_keybindings(&self) -> Vec<&Key> {
        vec![
            &self.quit,
            &self.play,
            &self.up,
            &self.down,
            &self.go_to_top,
            &self.go_to_bottom,
            &self.add_to_top_of_queue,
            &self.add_to_bottom_of_queue,
            &self.remove_from_queue,
        ]
    }

    pub fn pull_input<B: Backend>(&self, handler: &mut EventHandler<B>, instance: &mut Selection) {
        if poll(Duration::from_millis(1)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = read() {
                if key_event.code == KeyCode::Char(self.quit.key()) {
                    trigger(UniversalEvent::QuitProgram, handler, instance)
                } else if key_event.code == KeyCode::Char(self.play.key()) {
                    trigger(SelectionEvent::PlayQueue, handler, instance)
                } else if key_event.code == KeyCode::Char(self.up.key()) {
                    trigger(SelectionEvent::MoveUp, handler, instance)
                } else if key_event.code == KeyCode::Char(self.down.key()) {
                    trigger(SelectionEvent::MoveDown, handler, instance)
                } else if key_event.code == KeyCode::Char(self.go_to_top.key()) {
                    trigger(SelectionEvent::MoveToTop, handler, instance)
                } else if key_event.code == KeyCode::Char(self.go_to_bottom.key()) {
                    trigger(SelectionEvent::MoveToBottom, handler, instance)
                } else if key_event.code == KeyCode::Char(self.add_to_top_of_queue.key()) {
                    trigger(SelectionEvent::AddToTopOfQueue, handler, instance)
                } else if key_event.code == KeyCode::Char(self.add_to_bottom_of_queue.key()) {
                    trigger(SelectionEvent::AddToBottomOfQueue, handler, instance)
                } else if key_event.code == KeyCode::Char(self.remove_from_queue.key()) {
                    trigger(SelectionEvent::RemoveFromQueue, handler, instance)
                }
            }
        }
    }
}
