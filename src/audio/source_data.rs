use rodio::{Decoder, Source};
use std::{fs::File, time::Duration};

pub struct SourceData {
    pub samples: Vec<f32>,
    pub duration: Duration,
    pub path: String,
}

impl SourceData {
    pub fn new(path: &str) -> Option<SourceData> {
        let (duration, samples) = match get_source_data(path) {
            Some(data) => data,
            None => return None,
        };

        Some(SourceData {
            duration,
            samples,
            path: path.to_owned(),
        })
    }

    pub fn get_source(path: &str) -> Option<Decoder<File>> {
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        match Decoder::new(file) {
            Ok(source) => Some(source),
            Err(_) => return None,
        }
    }
}

fn get_source_data(path: &str) -> Option<(Duration, Vec<f32>)> {
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
