use std::{
    fs::{create_dir_all, read_dir},
    io, thread,
    time::Duration,
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{ListItem, ListState},
    Terminal,
};

use crate::{
    audio::{init::init_audio_handler, source_data::SourceData},
    input::{args::Args, config::Config, input::pull_input_while_listing},
    instance::audio_instance::AudioInstance,
    properties::runtime_properties::RuntimeOptions,
    render::list::draw_list,
};

pub fn run(config: Config, args: Args) -> Result<(), &'static str> {
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
                None => match create_dir_all(config.clone().audio_directory) {
                    Ok(_) => match read_dir(config.clone().audio_directory) {
                        Ok(paths) => paths
                            .map(|path| match path {
                                Ok(path) => path.path().display().to_string(),
                                Err(_) => "".to_owned(),
                            })
                            .collect(),
                        Err(_) => return Err("Failed to open default audio directory"),
                    },
                    Err(_) => return Err("Failed to create default audio directory"),
                },
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
                            config.get_color(),
                            config.get_highlight_color(),
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

    Ok(())
}
