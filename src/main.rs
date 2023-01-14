use std::{io, thread, time::Duration};

use audio::{init::init_audio_handler, source_data::SourceData};
use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use events::{args::Args, input::pull_input_while_listing};
use instance::audio_instance::AudioInstance;
use properties::runtime_properties::RuntimeOptions;
use render::list::draw_list;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{ListItem, ListState},
    Terminal,
};

mod audio;
mod events;
mod instance;
mod properties;
mod render;

fn main() -> Result<(), &'static str> {
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
                None => vec![
                    "mock_audio/phonk.mp3".to_owned(),
                    "mock_audio/rick.mp3".to_owned(),
                ],
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
                            tui::style::Color::Red,
                            tui::style::Color::LightCyan,
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

                pull_input_while_listing(&mut list_state, items.clone());
                thread::sleep(Duration::from_millis(interval));
            }
        }
    };

    match disable_raw_mode() {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to disable raw keyboard mod"),
    }
}
