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

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::enable_raw_mode,
};

use crate::{audio::source_data::SourceData, unicode::colors::get_color};

use super::{
    bar::draw_bar,
    chart::{create_data_from_samples, draw_chart},
    title::draw_title,
};

pub fn play_song<B: Backend>(sink: Sink, source_data: SourceData, terminal: &mut Terminal<B>) {
    terminal.clear().unwrap();
    enable_raw_mode().unwrap();

    let SourceData {
        source,
        duration,
        path,
        mut volume,
        samples,
        speed,
    } = source_data;

    let terminal_size = match terminal.size() {
        Ok(size) => size,
        Err(_) => panic!("size boogaloo"),
    };

    let color = get_color(true);
    let interval = 16;
    let sample_rate = source.sample_rate();
    let step = (sample_rate * interval) as f32 / 1000.0;
    let duration_secs = duration.as_secs_f32() / speed;

    sink.set_speed(speed);
    sink.set_volume(volume);
    sink.append(source.repeat_infinite());

    let start_time = Instant::now();

    loop {
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
                        // rect.render_widget(
                        //     draw_chart(data.as_slice(), max, min_sample_size, color),
                        //     chunks[0],
                        // );
                    }
                    None => {}
                };

                // rect.render_widget(draw_title(path.as_str(), color), chunks[1]);
                // rect.render_widget(draw_bar(progress, color), chunks[2]);
            })
            .unwrap();

        if poll(Duration::from_millis(10)).unwrap_or(false) {
            match read() {
                Ok(read_event) => match read_event {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Up => {
                            if volume <= 0.95 {
                                println!("up");
                                volume += 0.05;
                                sink.set_volume(volume)
                            }
                        }
                        KeyCode::Down => {
                            println!("down");
                            volume -= 0.05;
                            sink.set_volume(volume)
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Err(_) => {}
            }
        }
        thread::sleep(Duration::from_millis(interval.into()));
    }
}
