use std::io;

use audio::{init::init_audio_handler, source_data::SourceData};
use clap::Parser;
use events::args::Args;
use instance::audio_instance::AudioInstance;
use properties::runtime_properties::RuntimeOptions;
use tui::{backend::CrosstermBackend, Terminal};

mod audio;
mod events;
mod instance;
mod properties;
mod render;

fn main() -> Result<(), io::Error> {
    let (mut sink, _stream) = match init_audio_handler() {
        Some(handler_data) => handler_data,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to create audio sink",
            ))
        }
    };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut runtime_options = RuntimeOptions::new(50, 100);
    sink.set_speed(runtime_options.speed_decimal);
    sink.set_volume(runtime_options.volume_decimal);

    let args = Args::parse();
    let paths: Vec<String> = match args.play {
        Some(audio) => audio,
        None => match args.load {
            Some(audio) => audio,
            None => vec![
                "mock_audio/phonk.mp3".to_owned(),
                "mock_audio/rick.mp3".to_owned(),
            ],
        },
    };

    for path in paths {
        match AudioInstance::new(path.as_str()) {
            Some(mut instance) => {
                let source = match SourceData::get_source(path.as_str()) {
                    Some(source) => source,
                    None => break,
                };

                match instance.play_audio(&mut sink, source, &mut runtime_options, &mut terminal) {
                    Ok(_) => {}
                    Err(err) => println!("{err}"),
                };
            }
            None => {}
        };
    }

    Ok(())
}
