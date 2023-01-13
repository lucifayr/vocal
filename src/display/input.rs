use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use rodio::Sink;

use super::runtime::RuntimeOptions;

pub fn pull_input(sink: &Sink, options: &mut RuntimeOptions) {
    if poll(Duration::from_millis(1)).unwrap_or(false) {
        match read() {
            Ok(read_event) => match read_event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char(' ') => {
                        options.is_paused = !options.is_paused;
                        if options.is_paused {
                            sink.pause();
                        } else {
                            sink.play();
                        }
                    }
                    KeyCode::Up => {
                        if options.volume < 100 {
                            options.volume += 5;
                            options.volume_decimal = options.volume as f32 / 100.0;
                            if !options.is_muted {
                                sink.set_volume(options.volume_decimal);
                            }
                        }
                    }
                    KeyCode::Down => {
                        if options.volume > 0 {
                            options.volume -= 5;
                            options.volume_decimal = options.volume as f32 / 100.0;
                            if !options.is_muted {
                                sink.set_volume(options.volume_decimal);
                            }
                        }
                    }
                    KeyCode::Char('k') => {
                        if options.speed < 200 {
                            options.speed += 5;
                            options.speed_decimal = options.speed as f32 / 100.0;
                            sink.set_speed(options.speed_decimal);
                        }
                    }
                    KeyCode::Char('j') => {
                        if options.speed > 50 {
                            options.speed -= 5;
                            options.speed_decimal = options.speed as f32 / 100.0;
                            sink.set_speed(options.speed_decimal);
                        }
                    }
                    KeyCode::Char('r') => {
                        options.speed = 100;
                        options.speed_decimal = options.speed as f32 / 100.0;
                        sink.set_speed(options.speed_decimal);
                    }
                    KeyCode::Char('m') => {
                        if !options.is_muted {
                            sink.set_volume(0.0);
                            options.is_muted = true;
                        } else {
                            sink.set_volume(options.volume_decimal);
                            options.is_muted = false;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            Err(_) => {}
        }
    }
}
