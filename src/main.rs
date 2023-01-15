use std::{io, thread, time::Duration};

use audio::{init::init_audio_handler, source_data::SourceData};
use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use input::{args::Args, input::pull_input_while_listing};
use instance::audio_instance::AudioInstance;
use properties::runtime_properties::RuntimeOptions;
use render::{colors::get_color, list::draw_list};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{ListItem, ListState},
    Terminal,
};

use crate::input::config::Config;

mod audio;
mod input;
mod instance;
mod properties;
mod render;

fn main() -> Result<(), &'static str> {
    let config = match confy::load("vocal", "config") {
        Ok(config) => config,
        Err(_) => {
            let config = Config::default();
            match confy::store("vocal", "config", config.clone()) {
                Ok(_) => config,
                Err(_) => return Err("Failed to load config"),
            }
        }
    };

    match enable_raw_mode() {
        Ok(_) => {}
        Err(_) => return Err("Failed to enable raw keyboard mod"),
    }

    let (mut sink, _stream) = match init_audio_handler() {
        Some(handler_data) => handler_data,
        None => return Err("Failed to create audio sink"),
    };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(_) => return Err("Failed to create a TUI terminal"),
    };

    let mut runtime_options = RuntimeOptions::new(50, 100);
    sink.set_speed(runtime_options.speed_decimal);
    sink.set_volume(runtime_options.volume_decimal);

    let args = Args::parse();
    match args.play {
        Some(paths) => {
            for path in paths {
                match AudioInstance::new(path.as_str()) {
                    Some(mut instance) => {
                        let source = match SourceData::get_source(path.as_str()) {
                            Some(source) => source,
                            None => break,
                        };

                        match instance.play_audio(
                            &mut sink,
                            source,
                            &mut runtime_options,
                            &config,
                            &mut terminal,
                        ) {
                            Ok(_) => {}
                            Err(err) => println!("{err}"),
                        };
                    }
                    None => {}
                };
            }
        }
        None => {
            match terminal.clear() {
                Ok(_) => {}
                Err(_) => return Err("Failed to clear terminal"),
            }

            let paths = match args.load {
                Some(audio) => audio,
                None => vec![],
            };

            let items: Vec<ListItem> = paths
                .iter()
                .map(|path| ListItem::new(path.as_str()))
                .collect();

            let mut list_state = ListState::default();
            list_state.select(Some(0));

            let interval = 16_u64;

            loop {
                match terminal.draw(|rect| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .split(rect.size());

                    rect.render_stateful_widget(
                        draw_list(
                            items.clone(),
                            get_color(config.color.as_str()),
                            get_color(config.highlight_color.as_str()),
                        ),
                        chunks[0],
                        &mut list_state,
                    );
                }) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Failed to render frame: {}", err);
                    }
                }

                pull_input_while_listing(
                    &mut list_state,
                    paths.clone(),
                    &mut sink,
                    &mut runtime_options,
                    &config,
                    &mut terminal,
                );
                thread::sleep(Duration::from_millis(interval));
            }
        }
    };

    match disable_raw_mode() {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to disable raw keyboard mod"),
    }
}
