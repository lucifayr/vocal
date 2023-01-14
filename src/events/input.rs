use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use rodio::Sink;
use tui::widgets::ListState;

use crate::properties::{audio_properties::AudioOptions, runtime_properties::RuntimeOptions};

pub fn pull_input_while_playing(
    sink: &Sink,
    runtime_options: &mut RuntimeOptions,
    audio_options: &mut AudioOptions,
) {
    if poll(Duration::from_millis(1)).unwrap_or(false) {
        match read() {
            Ok(read_event) => match read_event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char(' ') => {
                        audio_options.is_paused = !audio_options.is_paused;
                        if audio_options.is_paused {
                            sink.pause();
                        } else {
                            sink.play();
                        }
                    }
                    KeyCode::Up => {
                        if runtime_options.volume < 100 {
                            runtime_options.volume += 5;
                            runtime_options.volume_decimal = runtime_options.volume as f32 / 100.0;
                            if !runtime_options.is_muted {
                                sink.set_volume(runtime_options.volume_decimal);
                            }
                        }
                    }
                    KeyCode::Down => {
                        if runtime_options.volume > 0 {
                            runtime_options.volume -= 5;
                            runtime_options.volume_decimal = runtime_options.volume as f32 / 100.0;
                            if !runtime_options.is_muted {
                                sink.set_volume(runtime_options.volume_decimal);
                            }
                        }
                    }
                    KeyCode::Char('k') => {
                        if runtime_options.speed < 200 {
                            runtime_options.speed += 5;
                            runtime_options.speed_decimal = runtime_options.speed as f32 / 100.0;
                            sink.set_speed(runtime_options.speed_decimal);
                        }
                    }
                    KeyCode::Char('j') => {
                        if runtime_options.speed > 50 {
                            runtime_options.speed -= 5;
                            runtime_options.speed_decimal = runtime_options.speed as f32 / 100.0;
                            sink.set_speed(runtime_options.speed_decimal);
                        }
                    }
                    KeyCode::Char('r') => {
                        runtime_options.speed = 100;
                        runtime_options.speed_decimal = runtime_options.speed as f32 / 100.0;
                        sink.set_speed(runtime_options.speed_decimal);
                    }
                    KeyCode::Char('m') => {
                        if !runtime_options.is_muted {
                            sink.set_volume(0.0);
                            runtime_options.is_muted = true;
                        } else {
                            sink.set_volume(runtime_options.volume_decimal);
                            runtime_options.is_muted = false;
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

pub fn pull_input_while_listing<T>(list_state: &mut ListState, items: Vec<T>) {
    if poll(Duration::from_millis(1)).unwrap_or(false) {
        match read() {
            Ok(read_event) => match read_event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Up => {
                        if let Some(selected) = list_state.selected() {
                            let amount = items.len();
                            if selected > 0 {
                                list_state.select(Some(selected - 1));
                            } else {
                                list_state.select(Some(amount - 1));
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(selected) = list_state.selected() {
                            let amount = items.len();
                            if selected >= amount - 1 {
                                list_state.select(Some(0));
                            } else {
                                list_state.select(Some(selected + 1));
                            }
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
