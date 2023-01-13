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
        samples,
        mut volume,
        mut speed,
    } = source_data;

    let mut is_muted = false;
    let mut is_paused = false;
    let mut time_since_last_pause_tick = Instant::now();
    let mut paused_time = 0.0;

    let mut volume_decimal = volume as f32 / 100.0;
    let mut speed_decimal = speed as f32 / 100.0;

    let terminal_size = match terminal.size() {
        Ok(size) => size,
        Err(_) => panic!("size boogaloo"),
    };

    let interval = 16;
    let sample_rate = source.sample_rate();
    let step = (sample_rate * interval) as f32 / 1000.0;
    let mut duration_secs = duration.as_secs_f64() / speed_decimal as f64;

    sink.set_speed(speed_decimal);
    sink.set_volume(volume_decimal);
    sink.append(source.repeat_infinite());

    let start_time = Instant::now();

    loop {
        let color = get_color(false);

        let passed_time = start_time.elapsed().as_secs_f64() - paused_time;
        if duration_secs < passed_time {
            return;
        }

        let progress = passed_time / duration_secs;

        let start = (progress * samples.len() as f64) as usize;
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
                        rect.render_widget(
                            draw_chart(data.as_slice(), max, min_sample_size, color),
                            chunks[0],
                        );
                    }
                    None => {}
                };

                rect.render_widget(draw_title(path.as_str(), color), chunks[1]);
                rect.render_widget(draw_bar(progress, color), chunks[2]);
            })
            .unwrap();

        loop {
            if poll(Duration::from_millis(1)).unwrap_or(false) {
                match read() {
                    Ok(read_event) => match read_event {
                        Event::Key(key_event) => match key_event.code {
                            KeyCode::Char(' ') => {
                                is_paused = !is_paused;
                                if is_paused {
                                    time_since_last_pause_tick = Instant::now();
                                    sink.pause();
                                } else {
                                    sink.play();
                                }
                            }
                            KeyCode::Up => {
                                if volume < 100 {
                                    volume += 5;
                                    volume_decimal = volume as f32 / 100.0;
                                    if !is_muted {
                                        sink.set_volume(volume_decimal);
                                    }
                                }
                            }
                            KeyCode::Down => {
                                if volume > 0 {
                                    volume -= 5;
                                    volume_decimal = volume as f32 / 100.0;
                                    if !is_muted {
                                        sink.set_volume(volume_decimal);
                                    }
                                }
                            }
                            KeyCode::Char('k') => {
                                if speed < 200 {
                                    speed += 5;
                                    speed_decimal = speed as f32 / 100.0;
                                    duration_secs = duration.as_secs_f64() / speed_decimal as f64;
                                    sink.set_speed(speed_decimal);
                                }
                            }
                            KeyCode::Char('j') => {
                                if speed > 50 {
                                    speed -= 5;
                                    speed_decimal = speed as f32 / 100.0;
                                    duration_secs = duration.as_secs_f64() / speed_decimal as f64;
                                    sink.set_speed(speed_decimal);
                                }
                            }
                            KeyCode::Char('r') => {
                                speed = 100;
                                speed_decimal = speed as f32 / 100.0;
                                duration_secs = duration.as_secs_f64() / speed_decimal as f64;
                                sink.set_speed(speed_decimal);
                            }
                            KeyCode::Char('m') => {
                                if !is_muted {
                                    sink.set_volume(0.0);
                                    is_muted = true;
                                } else {
                                    sink.set_volume(volume_decimal);
                                    is_muted = false;
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    Err(_) => {}
                }
            }
            if !is_paused {
                break;
            } else {
                paused_time += time_since_last_pause_tick.elapsed().as_secs_f64();
                time_since_last_pause_tick = Instant::now();
            }
        }
        thread::sleep(Duration::from_millis(interval.into()));
    }
}
