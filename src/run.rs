use std::io;

use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    audio::init::init_audio_handler,
    events::handler::EventHandler,
    input::{args::Args, config::Config},
    instance::{queue::Queue, selection::Selection, Instance},
    state::runtime_state::RuntimeState,
};

pub fn run(config: Config, args: Args) -> Result<(), &'static str> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(_) => return Err("Failed to create a TUI terminal"),
    };

    let volume = config.starting_volume.clamp(0, 100);
    let speed = config.starting_speed.clamp(10, 200);
    let state = RuntimeState::new(volume, speed);

    match args.play {
        Some(paths) => {
            let (sink, _stream) = match init_audio_handler() {
                Some(handler_data) => handler_data,
                None => return Err("Failed to create audio sink"),
            };

            sink.set_volume(state.get_volume_decimal());
            sink.set_speed(state.get_speed_decimal());

            let mut queue = Queue::new(paths, sink);
            let mut handler = EventHandler::new(state, config, terminal);
            queue.run(&mut handler);
        }
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

            let mut selection = Selection::new(paths);
            let mut handler = EventHandler::new(state, config, terminal);
            selection.run(&mut handler);
        }
    };
    Ok(())
}
