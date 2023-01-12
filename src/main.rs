use audio::init::{init_audio_handler, init_audio_source};
use display::play_song::play_song;

mod audio;
mod display;
mod unicode;

fn main() {
    let (x, _) = termion::terminal_size().unwrap();

    let (sink, _stream) = match init_audio_handler() {
        Some(handler_data) => handler_data,
        None => {
            println!("Failed to create audio sink");
            return;
        }
    };

    let path = "mock_audio/rick.mp3";
    let source_data = match init_audio_source(path) {
        Some(source_data) => source_data,
        None => {
            println!("Failed to get audio source");
            return;
        }
    };

    play_song(sink, source_data, x.into());
}
