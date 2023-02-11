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

use super::Instance;

pub struct Selection {
    pub content: Vec<String>,
    pub queue: Vec<String>,
    pub state: ListState,
}

impl Instance for Selection {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B, Self>) {
        handler.clear_terminal().unwrap();

        let keybindings = SelectionKeybindings::default();

        let items: Vec<ListItem> = self
            .content
            .iter()
            .map(|path| ListItem::new(get_filename_from_path(path.as_str()).unwrap_or("???")))
            .collect();

        let interval = 16_u64;

        loop {
            let queue_items: Vec<ListItem> = self
                .queue
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
                            handler.get_config().audio_directory.as_str(),
                            handler.get_config().get_color(),
                        ),
                        rect.size(),
                    );

                    return;
                }

                rect.render_stateful_widget(
                    draw_list(
                        items.clone(),
                        "Audio",
                        handler.get_config().get_color(),
                        handler.get_config().get_highlight_color(),
                    ),
                    chunks_horizontal[0],
                    &mut self.state,
                );

                rect.render_widget(
                    draw_list(
                        queue_items.clone(),
                        "Queue",
                        handler.get_config().get_color(),
                        handler.get_config().get_highlight_color(),
                    ),
                    chunks_horizontal[1],
                );

                rect.render_widget(
                    draw_keys(
                        keybindings.get_keybindings(),
                        handler.get_config().get_color(),
                        handler.get_config().get_highlight_color(),
                    ),
                    chunks_vertical[1],
                );
            }) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to render frame: {}", err);
                }
            }

            // keybindings.pull_input(handler);
            thread::sleep(Duration::from_millis(interval));
        }
    }
}

impl Selection {
    pub fn new(content: Vec<String>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Selection {
            content,
            state,
            queue: vec![],
        }
    }
}
