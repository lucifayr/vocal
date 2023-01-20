use std::{thread, time::Duration};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{ListItem, ListState},
};

use crate::{
    events::handler::EventHandler,
    input::selection_keybindings::SelectionKeybindings,
    render::{
        info::{draw_info_no_audio, get_filename_from_path},
        keybindings::draw_keys,
        list::draw_list,
    },
};

pub struct SelectionInstance {
    pub content: Vec<String>,
    pub queue: Vec<String>,
    pub state: ListState,
}

impl SelectionInstance {
    pub fn new(content: Vec<String>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        SelectionInstance {
            content,
            state,
            queue: vec![],
        }
    }

    pub fn show_selection<B: Backend>(handler: &mut EventHandler<B>) -> Result<(), &'static str> {
        match handler.terminal.clear() {
            Ok(_) => {}
            Err(_) => return Err("Failed to clear terminal"),
        }

        let keybindings = SelectionKeybindings::default();

        let content = handler
            .selection_instance
            .as_ref()
            .expect("Selection instance should exist")
            .content
            .clone();

        let items: Vec<ListItem> = content
            .iter()
            .map(|path| ListItem::new(get_filename_from_path(path.as_str()).unwrap_or("???")))
            .collect();

        let interval = 16_u64;

        loop {
            let queue = handler
                .selection_instance
                .as_ref()
                .expect("Selection instance should exist")
                .queue
                .clone();

            let queue_items: Vec<ListItem> = queue
                .iter()
                .map(|path| ListItem::new(get_filename_from_path(path.as_str()).unwrap_or("???")))
                .collect();

            match handler.terminal.draw(|rect| {
                let chunks_vertical = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
                    .split(rect.size());

                let chunks_horizontal = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                    .split(chunks_vertical[0]);

                if items.is_empty() {
                    rect.render_widget(
                        draw_info_no_audio(
                            handler.config.audio_directory.as_str(),
                            handler.config.get_color(),
                        ),
                        rect.size(),
                    )
                } else {
                    rect.render_stateful_widget(
                        draw_list(
                            items.clone(),
                            "Audio",
                            handler.config.get_color(),
                            handler.config.get_highlight_color(),
                        ),
                        chunks_horizontal[0],
                        &mut handler
                            .selection_instance
                            .as_mut()
                            .expect("Selection instance should exist")
                            .state,
                    );
                    rect.render_widget(
                        draw_list(
                            queue_items.clone(),
                            "Queue",
                            handler.config.get_color(),
                            handler.config.get_highlight_color(),
                        ),
                        chunks_horizontal[1],
                    );
                    rect.render_widget(
                        draw_keys(
                            keybindings.get_keybindings(),
                            handler.config.get_color(),
                            handler.config.get_highlight_color(),
                        ),
                        chunks_vertical[1],
                    );
                }
            }) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to render frame: {}", err);
                }
            }

            keybindings.pull_input(handler);
            thread::sleep(Duration::from_millis(interval));
        }
    }
}
