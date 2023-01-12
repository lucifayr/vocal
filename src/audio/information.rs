use rodio::{Decoder, Source};
use std::time::Duration;

pub fn get_duration_and_samples(path: &str) -> Option<(Duration, Vec<f32>)> {
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
    let samples: Vec<f32> = source.convert_samples().collect();

    let seconds = (samples.len() as f32 / sample_rate as f32) / channels as f32;
    Some((Duration::from_millis((seconds * 1000.0) as u64), samples))
}
