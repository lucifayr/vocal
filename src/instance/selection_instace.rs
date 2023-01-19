use std::{thread, time::Duration};

use rodio::Sink;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{ListItem, ListState},
    Terminal,
};

use crate::{
    input::{config::Config, selection_keybindings::SelectionKeybindings},
    properties::runtime_properties::RuntimeOptions,
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

    pub fn show_selection<B: Backend>(
        &mut self,
        sink: &mut Sink,
        runtime_options: &mut RuntimeOptions,
        config: &Config,
        terminal: &mut Terminal<B>,
    ) -> Result<(), &'static str> {
        match terminal.clear() {
            Ok(_) => {}
            Err(_) => return Err("Failed to clear terminal"),
        }

        let keybindings = SelectionKeybindings::default();

        let content = self.content.clone();
        let items: Vec<ListItem> = content
            .iter()
            .map(|path| ListItem::new(get_filename_from_path(path.as_str()).unwrap_or("???")))
            .collect();

        let interval = 16_u64;

        loop {
            let queue = self.queue.clone();
            let queue_items: Vec<ListItem> = queue
                .iter()
                .map(|path| ListItem::new(get_filename_from_path(path.as_str()).unwrap_or("???")))
                .collect();

            match terminal.draw(|rect| {
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
                        draw_info_no_audio(config.audio_directory.as_str(), config.get_color()),
                        rect.size(),
                    )
                } else {
                    rect.render_stateful_widget(
                        draw_list(
                            items.clone(),
                            "Audio",
                            config.get_color(),
                            config.get_highlight_color(),
                        ),
                        chunks_horizontal[0],
                        &mut self.state,
                    );
                    rect.render_widget(
                        draw_list(
                            queue_items.clone(),
                            "Queue",
                            config.get_color(),
                            config.get_highlight_color(),
                        ),
                        chunks_horizontal[1],
                    );
                    rect.render_widget(
                        draw_keys(
                            keybindings.get_keybindings(),
                            config.get_color(),
                            config.get_highlight_color(),
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

            keybindings.pull_input(self, sink, runtime_options, config, terminal);
            thread::sleep(Duration::from_millis(interval));
        }
    }
}
