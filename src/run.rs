use std::io;

use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    audio::init::init_audio_handler,
    events::handler::EventHandler,
    input::{args::Args, config::Config},
    instance::{audio_instance::AudioInstance, selection_instace::SelectionInstance},
    properties::runtime_properties::RuntimeOptions,
};

pub fn run(config: Config, args: Args) -> Result<(), &'static str> {
    let (sink, _stream) = match init_audio_handler() {
        Some(handler_data) => handler_data,
        None => return Err("Failed to create audio sink"),
    };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(_) => return Err("Failed to create a TUI terminal"),
    };

    let volume = config.starting_volume.clamp(0, 100);
    let speed = config.starting_speed.clamp(10, 200);
    let runtime_options = RuntimeOptions::new(volume, speed);

    sink.set_speed(runtime_options.speed_decimal);
    sink.set_volume(runtime_options.volume_decimal);

    let mut handler = EventHandler::new(&sink, runtime_options);

    match args.play {
        Some(paths) => AudioInstance::play_queue(paths, &config, &mut terminal, &mut handler),
        None => {
            let paths = match args.load {
                Some(audio) => audio,
                None => {
                    match Config::get_audio_directory_content(config.audio_directory.as_str()) {
                        Ok(paths) => paths,
                        Err(err) => return Err(err),
                    }
                }
            };

            let mut selection_instance = SelectionInstance::new(paths);

            match selection_instance.show_selection(&config, &mut terminal, &mut handler) {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }
    };
    Ok(())
}
