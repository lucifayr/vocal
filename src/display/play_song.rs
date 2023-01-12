use std::{
    thread,
    time::{Duration, Instant},
};

use rodio::{Sink, Source};
use termion::{clear, cursor};

use crate::{
    audio::{graph::render_bar_graph, source_data::SourceData},
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
        samples,
    } = source_data;

    let name = match get_filename_from_path(path.as_str()) {
        Some(name) => name,
        None => "???",
    };

    let color = unicode::colors::Color::Blue;

    let interval = 20;
    let sample_rate = source.sample_rate();
    let step = (sample_rate * interval) as f32 / 1000.0;

    sink.set_speed(speed);
    sink.set_volume(volume);
    sink.append(source);

    let start_time = Instant::now();
    loop {
        print!("{}{}", cursor::Goto(1, 1), clear::All);

        let mut content = "".to_owned();

        let passed_time = start_time.elapsed().as_secs_f32();
        let progress = passed_time / duration.as_secs_f32();

        let start = (progress * samples.len() as f32) as usize;

        let sample_slice: Vec<f32> = samples
            .clone()
            .iter()
            .skip(start)
            .take(step as usize)
            .map(|s| *s)
            .collect();

        let bar_count = (terminal_data.x / 2) as usize;
        let chunk_size = sample_slice.len() / bar_count;

        let reduced_samples: Vec<f32> = sample_slice
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().sum::<f32>() / chunk_size as f32)
            .take(bar_count)
            .collect();

        let title = format!("Playing: {}\n\n", name);

        content += render_bar_graph(reduced_samples, &terminal_data, bar_count, color).as_str();
        content += format!(
            "{}",
            cursor::Goto(
                terminal_data.x / 2 - title.len() as u16 / 2,
                terminal_data.y / 2 + 2
            )
        )
        .as_str();
        content += title.as_str();
        content += render_loading_bar(
            passed_time,
            0.0,
            duration.as_secs_f32() / speed,
            terminal_data.x.into(),
            color,
        )
        .as_str();

        println!("{content}");
        thread::sleep(Duration::from_millis(interval.into()));
    }
}
