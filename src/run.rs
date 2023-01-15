use std::{io, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{ListItem, ListState},
    Terminal,
};

use crate::{
    audio::init::init_audio_handler,
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
                AudioInstance::start_instance(
                    path,
                    &mut sink,
                    &mut runtime_options,
                    &config,
                    &mut terminal,
                )
            }
        }
        None => {
            match terminal.clear() {
                Ok(_) => {}
                Err(_) => return Err("Failed to clear terminal"),
            }

            let paths = match args.load {
                Some(audio) => audio,
                None => {
                    match Config::get_audio_directory_content(config.audio_directory.as_str()) {
                        Ok(paths) => paths,
                        Err(err) => return Err(err),
                    }
                }
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
