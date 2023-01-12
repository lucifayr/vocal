use std::{
    thread,
    time::{Duration, Instant},
};

use rodio::Sink;

use crate::{
    audio::source_data::SourceData,
    unicode::{self, render::render_loading_bar},
};

pub fn play_song(sink: Sink, source_data: SourceData, width: i32) {
    let SourceData { source, duration } = source_data;

    sink.append(source);
    let start_time = Instant::now();
    loop {
        let passed_time = start_time.elapsed().as_secs_f32();
        let bar = render_loading_bar(
            passed_time,
            0.0,
            duration.as_secs_f32(),
            width,
            unicode::colors::Color::Blue,
        );

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{bar}");

        thread::sleep(Duration::from_millis(50));
    }
}
