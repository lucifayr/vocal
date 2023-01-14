use std::{
    thread,
    time::{Duration, Instant},
};

use rodio::{Sink, Source};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::{
    audio::source_data::SourceData,
    events::input::pull_input,
    properties::{audio_properties::AudioOptions, runtime_properties::RuntimeOptions},
};

use super::{
    bar::draw_bar,
    chart::{create_data_from_samples, draw_chart},
    colors::get_color,
    info::draw_info,
};

pub fn play_song<B: Backend>(
    sink: Sink,
    source_data: SourceData,
    terminal: &mut Terminal<B>,
    runtime_options: &mut RuntimeOptions,
) {
    terminal.clear().unwrap();
    enable_raw_mode().unwrap();

    let terminal_size = match terminal.size() {
        Ok(size) => size,
        Err(_) => panic!("size boogaloo"),
    };

    let SourceData {
        source,
        path,
        samples,
        duration,
    } = source_data;

    let mut audio_options = AudioOptions::new(duration);

    let interval = 16;
    let sample_rate = source.sample_rate();
    let step = (sample_rate * interval) as f32 / 1000.0;

    sink.set_speed(runtime_options.speed_decimal);
    sink.set_volume(runtime_options.volume_decimal);
    sink.append(source);

    loop {
        let color = get_color(false);

        audio_options.passed_time += audio_options.time_since_last_tick.elapsed().as_secs_f64()
            * runtime_options.speed_decimal as f64;

        audio_options.time_since_last_tick = Instant::now();

        let progress = audio_options.passed_time / audio_options.duration.as_secs_f64();
        if progress > 1.0 {
            disable_raw_mode().unwrap();
            return;
        }

        let start = (progress * samples.len() as f64) as usize;
        let bar_count = (terminal_size.width / 2) as usize;

        match terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(size.height / 2),
                        Constraint::Length((3 * size.height) / 8),
                        Constraint::Max(size.height / 8),
                    ]
                    .as_ref(),
                )
                .split(size);

            let max = 10000;
            let min_sample_size = 0.3;

            match create_data_from_samples(
                samples.clone(),
                start,
                step as usize,
                bar_count,
                max,
                min_sample_size,
            ) {
                Some(data) => {
                    rect.render_widget(
                        draw_chart(data.as_slice(), max, min_sample_size, color),
                        chunks[0],
                    );
                }
                None => {}
            };

            rect.render_widget(
                draw_info(
                    path.as_str(),
                    runtime_options.volume,
                    runtime_options.is_muted,
                    runtime_options.speed,
                    audio_options.duration.as_secs_f64(),
                    audio_options.passed_time,
                    color,
                ),
                chunks[1],
            );
            rect.render_widget(draw_bar(progress, color), chunks[2]);
        }) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to render frame: {}", err);
            }
        }

        loop {
            pull_input(&sink, runtime_options, &mut audio_options);
            if !audio_options.is_paused {
                break;
            }
        }
        thread::sleep(Duration::from_millis(interval.into()));
    }
}
