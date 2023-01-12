use rodio::{Decoder, OutputStream, Sink, Source};
use std::{
    thread,
    time::{Duration, Instant},
};

use crate::unicode::render::render_loading_bar;

mod unicode;

fn main() {
    let (x, _) = termion::terminal_size().unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let path = "mock_audio/rick.mp3";

    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(file).unwrap();

    let source_duration = source.total_duration();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let duration = match source_duration {
        Some(duration) => duration,
        None => get_duration(path),
    };

    sink.append(source);
    let start_time = Instant::now();
    loop {
        let passed_time = start_time.elapsed().as_secs_f32();
        let bar = render_loading_bar(
            passed_time,
            0.0,
            duration.as_secs_f32(),
            x.into(),
            unicode::colors::Color::Blue,
        );

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{bar}");

        thread::sleep(Duration::from_millis(100));
    }
}

fn get_duration(path: &str) -> Duration {
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(file).unwrap();

    let channels = source.channels();
    let sample_rate = source.sample_rate();
    let sample_count = source.count();

    let seconds = (sample_count as f32 / sample_rate as f32) / channels as f32;
    Duration::from_millis((seconds * 1000.0) as u64)
}
