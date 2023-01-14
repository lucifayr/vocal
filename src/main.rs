use std::io;

use audio::{init::init_audio_handler, source_data::SourceData};
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

    let paths = ["mock_audio/phonk.mp3", "mock_audio/rick.mp3"];

    for path in paths {
        match AudioInstance::new(path) {
            Some(mut instance) => {
                let source = match SourceData::get_source(path) {
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
