use std::{
    thread,
    time::{Duration, Instant},
};

use rodio::Sink;

use crate::{
    audio::source_data::SourceData,
    unicode::{self, render::render_loading_bar},
};

use super::{terminal::TerminalData, text::get_filename_from_path};

pub fn play_song(sink: Sink, source_data: SourceData, terminal_data: TerminalData) {
    let SourceData {
        source,
        duration,
        path,
        speed,
        volume,
    } = source_data;

    let name = match get_filename_from_path(path.as_str()) {
        Some(name) => name,
        None => "???",
    };

    sink.set_speed(speed);
    sink.set_volume(volume);
    sink.append(source);
    let start_time = Instant::now();
    loop {
        let passed_time = start_time.elapsed().as_secs_f32();
        let bar = render_loading_bar(
            passed_time,
            0.0,
            duration.as_secs_f32() / speed,
            terminal_data.x.into(),
            unicode::colors::Color::Blue,
        );

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Playing: {}\n", name);
        println!("{bar}");

        thread::sleep(Duration::from_millis(50));
    }
}
