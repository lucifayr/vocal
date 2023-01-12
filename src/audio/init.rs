use rodio::{Decoder, OutputStream, Sink, Source};

use super::{information::get_duration, source_data::SourceData};

pub fn init_audio_source(path: &str) -> Option<SourceData> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let source = match Decoder::new(file) {
        Ok(source) => source,
        Err(_) => return None,
    };

    let source_duration = source.total_duration();
    let duration = match source_duration {
        Some(duration) => duration,
        None => match get_duration(path) {
            Some(duration) => duration,
            None => return None,
        },
    };

    Some(SourceData {
        source,
        duration,
        path: path.to_owned(),
        speed: 1.0,
        volume: 1.0,
    })
}

pub fn init_audio_handler() -> Option<(Sink, OutputStream)> {
    match OutputStream::try_default() {
        Ok(stream_data) => match Sink::try_new(&stream_data.1) {
            Ok(sink) => Some((sink, stream_data.0)),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
