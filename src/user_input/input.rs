use std::{process::exit, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::disable_raw_mode,
};
use rodio::Sink;
use tui::{backend::Backend, Terminal};

use crate::{
    instance::{audio_instance::AudioInstance, selection_instace::SelectionInstance},
    properties::{audio_properties::AudioOptions, runtime_properties::RuntimeOptions},
};

use super::config::Config;

pub fn pull_input_while_playing(
    sink: &Sink,
    runtime_options: &mut RuntimeOptions,
    audio_options: &mut AudioOptions,
) {
    if poll(Duration::from_millis(1)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = read() {
            match key_event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    exit(0);
                }
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
            }
        }
    }
}

pub fn pull_input_while_listing<B: Backend>(
    instance: &mut SelectionInstance,
    sink: &mut Sink,
    runtime_options: &mut RuntimeOptions,
    config: &Config,
    terminal: &mut Terminal<B>,
) {
    if poll(Duration::from_millis(1)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = read() {
            match key_event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    exit(0);
                }
                KeyCode::Enter => {
                    add_to_queue(instance, true);
                    play_queue(instance, sink, runtime_options, config, terminal);
                }
                KeyCode::Char('p') => play_queue(instance, sink, runtime_options, config, terminal),
                KeyCode::Up | KeyCode::Char('k') => move_up(instance),
                KeyCode::Down | KeyCode::Char('j') => move_down(instance),
                KeyCode::Right | KeyCode::Char('l') => add_to_queue(instance, false),
                KeyCode::Left | KeyCode::Char('h') => remove_from_queue(instance),
                _ => {}
            }
        }
    }
}

fn move_up(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        let amount = instance.content.len();
        if selected > 0 {
            instance.state.select(Some(selected - 1));
        } else {
            instance.state.select(Some(amount - 1));
        }
    }
}

fn move_down(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        let amount = instance.content.len();
        if selected >= amount - 1 {
            instance.state.select(Some(0));
        } else {
            instance.state.select(Some(selected + 1));
        }
    }
}

fn remove_from_queue(instance: &mut SelectionInstance) {
    if let Some(selected) = instance.state.selected() {
        if let Some(item) = instance.content.get(selected) {
            if let Some(index) = instance.queue.iter().position(|element| element == item) {
                instance.queue.remove(index);
            }
        }
    }
}

fn add_to_queue(instance: &mut SelectionInstance, at_start: bool) {
    if let Some(selected) = instance.state.selected() {
        if let Some(item) = instance.content.get(selected) {
            if !instance.queue.contains(item) {
                if at_start {
                    instance.queue.splice(0..0, vec![item.to_owned()]);
                } else {
                    instance.queue.push(item.to_owned())
                }
            }
        };
    }
}

fn play_queue<B: Backend>(
    instance: &mut SelectionInstance,
    sink: &mut Sink,
    runtime_options: &mut RuntimeOptions,
    config: &Config,
    terminal: &mut Terminal<B>,
) {
    instance.queue.iter().for_each(|path| {
        AudioInstance::start_instance(path.to_owned(), sink, runtime_options, config, terminal)
    })
}
