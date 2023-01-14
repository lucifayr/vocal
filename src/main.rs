use std::io;

use audio::{init::init_audio_handler, source_data::SourceData};
use properties::runtime_properties::RuntimeOptions;
use render::play_song::play_song;
use tui::{backend::CrosstermBackend, Terminal};

mod audio;
mod events;
mod properties;
mod render;

fn main() -> Result<(), io::Error> {
    let (sink, _stream) = match init_audio_handler() {
        Some(handler_data) => handler_data,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to create audio sink",
            ))
        }
    };

    let path = "mock_audio/rick.mp3";
    let source_data = match SourceData::new(path) {
        Some(source_data) => source_data,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get audio source",
            ))
        }
    };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut runtime_options = RuntimeOptions::new(50, 100);

    play_song(sink, source_data, &mut terminal, &mut runtime_options);
    Ok(())
}
