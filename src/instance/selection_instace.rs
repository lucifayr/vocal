use std::{thread, time::Duration};

use rodio::Sink;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{ListItem, ListState},
    Terminal,
};

use crate::{
    input::{config::Config, input::pull_input_while_listing},
    properties::runtime_properties::RuntimeOptions,
    render::{
        info::{draw_info_no_audio, get_filename_from_path},
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

        let content = self.content.clone();
        let items: Vec<ListItem> = content
            .iter()
            .map(|path| {
                ListItem::new(match get_filename_from_path(path.as_str()) {
                    Some(path) => path,
                    None => "???",
                })
            })
            .collect();

        let interval = 16_u64;

        loop {
            let queue = self.queue.clone();
            let queue_items: Vec<ListItem> = queue
                .iter()
                .map(|path| {
                    ListItem::new(match get_filename_from_path(path.as_str()) {
                        Some(path) => path,
                        None => "???",
                    })
                })
                .collect();

            match terminal.draw(|rect| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                    .split(rect.size());

                if items.len() == 0 {
                    rect.render_widget(
                        draw_info_no_audio(config.audio_directory.as_str(), config.get_color()),
                        chunks[0],
                    )
                } else {
                    rect.render_stateful_widget(
                        draw_list(
                            items.clone(),
                            "Audio",
                            config.get_color(),
                            config.get_highlight_color(),
                        ),
                        chunks[0],
                        &mut self.state,
                    );
                    rect.render_widget(
                        draw_list(
                            queue_items.clone(),
                            "Queue",
                            config.get_color(),
                            config.get_highlight_color(),
                        ),
                        chunks[1],
                    )
                }
            }) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to render frame: {}", err);
                }
            }

            pull_input_while_listing(self, sink, runtime_options, &config, terminal);
            thread::sleep(Duration::from_millis(interval));
        }
    }
}
