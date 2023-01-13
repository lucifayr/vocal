use std::{
    thread,
    time::{Duration, Instant},
};

use rodio::{Sink, Source};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{BarChart, Block, Borders, Gauge, Paragraph},
    Terminal,
};

use crate::{audio::source_data::SourceData, unicode::colors::get_color};

use super::text::get_filename_from_path;

pub fn play_song<B: Backend>(sink: Sink, source_data: SourceData, terminal: &mut Terminal<B>) {
    terminal.clear().unwrap();

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

    let terminal_size = match terminal.size() {
        Ok(size) => size,
        Err(_) => panic!("size boogaloo"),
    };

    let interval = 16;
    let sample_rate = source.sample_rate();
    let step = (sample_rate * interval) as f32 / 1000.0;
    let duration_secs = duration.as_secs_f32() / speed;

    sink.set_speed(speed);
    sink.set_volume(volume / 25.0);
    sink.append(source);

    let start_time = Instant::now();
    loop {
        let color = get_color(true);

        let passed_time = start_time.elapsed().as_secs_f32();
        if duration_secs < passed_time {
            return;
        }

        let progress = passed_time / duration_secs;

        let start = (progress * samples.len() as f32) as usize;
        let bar_count = (terminal_size.width / 2) as usize;

        terminal
            .draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length((3 * size.height) / 4),
                            Constraint::Length(size.height / 8),
                            Constraint::Length(size.height / 8),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let max = 10000;
                let min_sample_size = 0.3;
                let reduced_samples = match reduce_sample_to_slice(
                    samples.clone(),
                    start,
                    step as usize,
                    bar_count,
                    min_sample_size,
                ) {
                    Ok(samples) => samples,
                    Err(_) => return,
                };

                let data_samples: Vec<(&str, u64)> = reduced_samples
                    .iter()
                    .map(|sample| ("", (max as f32 * (1.0 + min_sample_size) * sample) as u64))
                    .collect();

                let data: &[(&str, u64)] = data_samples.as_slice();

                let chart = BarChart::default()
                    .block(Block::default().borders(Borders::BOTTOM))
                    .bar_width(3)
                    .bar_gap(1)
                    .bar_style(Style::default().fg(color).bg(Color::Reset))
                    .value_style(Style::default().fg(color).bg(color))
                    .label_style(Style::default())
                    .data(data)
                    .max((max as f32 * (1.0 + min_sample_size)) as u64);

                let title = Paragraph::new(Text::styled(
                    format!("Playing: {}\n\n", name),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ))
                .alignment(Alignment::Center);

                let bar_progress = (progress * 100.0) as u16;
                let bar = Gauge::default()
                    .block(Block::default())
                    .gauge_style(Style::default().fg(color).bg(Color::Reset))
                    .percent(bar_progress);

                rect.render_widget(chart, chunks[0]);
                rect.render_widget(title, chunks[1]);
                rect.render_widget(bar, chunks[2]);
            })
            .unwrap();

        // content += render_title(title.as_str(), &terminal_size).as_str();
        // content += render_loading_bar(
        //     passed_time,
        //     0.0,
        //     duration_secs,
        //     terminal_size.width.into(),
        //     color,
        // )
        // .as_str();

        // println!("{content}");
        thread::sleep(Duration::from_millis(interval.into()));
    }
}

fn reduce_sample_to_slice(
    samples: Vec<f32>,
    start: usize,
    step: usize,
    bar_count: usize,
    min_sample_size: f32,
) -> Result<Vec<f32>, ()> {
    let sample_slice: Vec<f32> = samples
        .clone()
        .iter()
        .skip(start)
        .take(step)
        .map(|s| *s + min_sample_size)
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
