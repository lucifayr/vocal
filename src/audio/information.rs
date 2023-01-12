use rodio::{Decoder, Source};
use std::time::Duration;

pub fn get_duration(path: &str) -> Option<Duration> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let source = match Decoder::new(file) {
        Ok(source) => source,
        Err(_) => return None,
    };

    let channels = source.channels();
    let sample_rate = source.sample_rate();
    let sample_count = source.count();

    let seconds = (sample_count as f32 / sample_rate as f32) / channels as f32;
    Some(Duration::from_millis((seconds * 1000.0) as u64))
}
