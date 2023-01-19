use std::{process::exit, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::disable_raw_mode,
};
use rodio::Sink;
use tui::{backend::Backend, Terminal};

use crate::{
    instance::{audio_instance::AudioInstance, selection_instace::SelectionInstance},
    properties::runtime_properties::RuntimeOptions,
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
            add_to_bottom_of_queue: Key::new("l", "add to bottom of queue"),
            add_to_top_of_queue: Key::new("L", "add to top of queue"),
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
        instance: &mut SelectionInstance,
        sink: &mut Sink,
        runtime_options: &mut RuntimeOptions,
        config: &Config,
        terminal: &mut Terminal<B>,
    ) {
        if poll(Duration::from_millis(1)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = read() {
                match key_event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode().unwrap();
                        exit(0);
                    }
                    KeyCode::Char('p') => {
                        AudioInstance::play_queue(
                            instance.queue.clone(),
                            sink,
                            runtime_options,
                            config,
                            terminal,
                        );
                    }
                    KeyCode::Up | KeyCode::Char('k') => move_up(instance),
                    KeyCode::Down | KeyCode::Char('j') => move_down(instance),
                    KeyCode::Right | KeyCode::Char('l') => add_to_bottom_of_queue(instance),
                    KeyCode::Char('L') => add_to_start_of_queue(instance),
                    KeyCode::Left | KeyCode::Char('h') => remove_from_queue(instance),
                    KeyCode::Char('g') => move_to_top(instance),
                    KeyCode::Char('G') => move_to_bottom(instance),
                    _ => {}
                }
            }
        }
    }
}

fn move_up(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        let amount = instance.content.len();
        if selected > 0 {
            instance.state.select(Some(selected - 1));
        } else {
            instance.state.select(Some(amount - 1));
        }
    }
}

fn move_down(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        let amount = instance.content.len();
        if selected >= amount - 1 {
            instance.state.select(Some(0));
        } else {
            instance.state.select(Some(selected + 1));
        }
    }
}

fn move_to_top(instance: &mut SelectionInstance) {
    instance.state.select(Some(0));
}
fn move_to_bottom(instance: &mut SelectionInstance) {
    let amount = instance.content.len();
    instance.state.select(Some(amount - 1));
}

fn remove_from_queue(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        if let Some(item) = instance.content.get(selected) {
            if let Some(index) = instance.queue.iter().position(|element| element == item) {
                instance.queue.remove(index);
            }
        }
    }
}

fn add_to_bottom_of_queue(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        if let Some(item) = instance.content.get(selected) {
            if !instance.queue.contains(item) {
                instance.queue.push(item.to_owned())
            }
        };
    }
}

fn add_to_start_of_queue(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        if let Some(item) = instance.content.get(selected) {
            if !instance.queue.contains(item) {
                instance.queue.splice(0..0, vec![item.to_owned()]);
            }
        };
    }
}
