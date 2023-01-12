use std::{
    thread,
    time::{Duration, Instant},
};

use rodio::{Sink, Source};
use termion::{clear, cursor};

use crate::{
    audio::{graph::render_bar_graph, source_data::SourceData},
    unicode::{
        self,
        render::{render_loading_bar, render_title},
    },
};

use super::{terminal::TerminalData, text::get_filename_from_path};

pub fn play_song(sink: Sink, source_data: SourceData, terminal_data: TerminalData) {
    let SourceData {
        source,
        duration,
        path,
        volume,
        samples,
        speed,
    } = source_data;

    let name = match get_filename_from_path(path.as_str()) {
        Some(name) => name,
        None => "???",
    };

    let color = unicode::colors::Color::Blue;

    let interval = 20;
    let sample_rate = source.sample_rate();
    let step = (sample_rate * interval) as f32 / 1000.0;
    let duration_secs = duration.as_secs_f32() / speed;

    sink.set_speed(speed);
    sink.set_volume(volume / 2.0);
    sink.append(source);

    let start_time = Instant::now();
    loop {
        print!("{}{}", cursor::Goto(1, 1), clear::All);
        let mut content = "".to_owned();

        let passed_time = start_time.elapsed().as_secs_f32();
        if duration_secs < passed_time {
            return;
        }

        let progress = passed_time / duration_secs;

        let start = (progress * samples.len() as f32) as usize;
        let bar_count = (terminal_data.x / 2) as usize;

        let reduced_samples =
            match reduce_sample_to_slice(samples.clone(), start, step as usize, bar_count) {
                Ok(samples) => samples,
                Err(_) => return,
            };

        let title = format!("Playing: {}\n\n", name);

        content += render_bar_graph(reduced_samples, &terminal_data, bar_count, color).as_str();
        content += render_title(title.as_str(), &terminal_data).as_str();
        content += render_loading_bar(
            passed_time,
            0.0,
            duration_secs,
            terminal_data.x.into(),
            color,
        )
        .as_str();

        println!("{content}");
        thread::sleep(Duration::from_millis(interval.into()));
    }
}

fn reduce_sample_to_slice(
    samples: Vec<f32>,
    start: usize,
    step: usize,
    bar_count: usize,
) -> Result<Vec<f32>, ()> {
    let sample_slice: Vec<f32> = samples
        .clone()
        .iter()
        .skip(start)
        .take(step)
        .map(|s| *s)
        .collect();

    let chunk_size = match sample_slice.len() / bar_count {
        x if x != 0 => x,
        _ => return Err(()),
    };

    Ok(sample_slice
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().sum::<f32>() / chunk_size as f32)
        .take(bar_count)
        .collect())
}
