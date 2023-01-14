use rodio::{OutputStream, Sink};

pub fn init_audio_handler() -> Option<(Sink, OutputStream)> {
    match OutputStream::try_default() {
        Ok(stream_data) => match Sink::try_new(&stream_data.1) {
            Ok(sink) => Some((sink, stream_data.0)),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
