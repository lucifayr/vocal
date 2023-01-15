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
    render::{info::draw_info_no_audio, list::draw_list},
};

pub struct SelectionInstance {
    pub content: Vec<String>,
    pub state: ListState,
}

impl SelectionInstance {
    pub fn new(content: Vec<String>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        SelectionInstance { content, state }
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

        let items: Vec<ListItem> = self
            .content
            .iter()
            .map(|path| ListItem::new(path.as_str()))
            .collect();

        let interval = 16_u64;

        loop {
            match terminal.draw(|rect| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)].as_ref())
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
                            config.get_color(),
                            config.get_highlight_color(),
                        ),
                        chunks[0],
                        &mut self.state,
                    );
                }
            }) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to render frame: {}", err);
                }
            }

            pull_input_while_listing(
                &mut self.state,
                self.content.clone(),
                sink,
                runtime_options,
                &config,
                terminal,
            );
            thread::sleep(Duration::from_millis(interval));
        }
    }
}
