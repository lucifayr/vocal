use std::io;

use audio::init::{init_audio_handler, init_audio_source};
use display::play_song::play_song;
use tui::{backend::CrosstermBackend, Terminal};

mod audio;
mod display;
mod unicode;

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

    let path = "mock_audio/phonk.mp3";
    let source_data = match init_audio_source(path) {
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

    play_song(sink, source_data, &mut terminal);
    Ok(())
}
