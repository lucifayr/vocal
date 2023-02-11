use std::{thread, time::Duration};

use crossterm::event::KeyCode;
use rodio::{OutputStream, Sink, Source};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    audio::source_data::SourceData,
    events::{
        audio_events::AudioEvent,
        handler::{trigger, EventHandler},
        queue_events::QueueEvent,
    },
    input::{
        key::{poll_key, Key},
        player_keybindings::{get_player_keybindings, process_player_input},
    },
    instance::{queue::Queue, Instance, InstanceRunableWithParent},
    render::{
        bar::draw_bar,
        chart::{create_data_from_samples, draw_chart},
        info::draw_info,
        keybindings::draw_keys,
    },
    state::audio_state::AudioState,
};

use super::init::init_audio_handler;

pub struct Player {
    pub sink: Sink,
    // ====================================================================================================
    // this field has to be stored because if it goes out of scope no audio
    // will play through the sink
    // =================================================================================================
    _stream: OutputStream,
    pub source_data: SourceData,
    pub state: AudioState,
}

impl InstanceRunableWithParent<Queue> for Player {
    fn run<B: Backend>(&mut self, handler: &mut EventHandler<B>, parent: &mut Queue) {
        trigger(AudioEvent::Start, handler, self);
        let terminal_size = handler.get_terminal_size().unwrap();

        let source = SourceData::get_source(&self.source_data.path).unwrap();
        let interval = 16;
        let sample_rate = source.sample_rate();
        let step = (sample_rate * interval) as f32 / 1000.0;

        self.sink.append(source);
        loop {
            trigger(AudioEvent::Tick, handler, self);

            if parent.interupted {
                trigger(AudioEvent::End, handler, self);
                trigger(QueueEvent::End, handler, parent);
                return;
            }

            let progress = self.state.progress;
            if progress > 1.0 {
                trigger(AudioEvent::End, handler, self);
                return;
            }

            let start = (progress * self.source_data.samples.len() as f64) as usize;
            let bar_count = (terminal_size.width / 2) as usize;

            let volume = handler.get_state().volume;
            let speed = handler.get_state().speed;
            let is_muted = handler.get_state().is_muted;
            let color = handler.get_config().get_color();
            let highlight_color = handler.get_config().get_highlight_color();

            match handler.terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(25),
                            Constraint::Percentage(10),
                            Constraint::Percentage(15),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let max = 10000;
                let multiplier = 100_f32;

                if let Some(data) = create_data_from_samples(
                    self.source_data.samples.clone(),
                    start,
                    step as usize,
                    bar_count,
                    max,
                    multiplier,
                ) {
                    rect.render_widget(
                        draw_chart(data.as_slice(), max * multiplier as u64, color),
                        chunks[0],
                    );
                }

                rect.render_widget(
                    draw_info(
                        &self.source_data.path,
                        volume,
                        is_muted,
                        speed,
                        self.state.duration.as_secs_f64(),
                        self.state.passed_time,
                        highlight_color,
                    ),
                    chunks[1],
                );
                rect.render_widget(draw_bar(progress, color), chunks[2]);

                let keybindings = [Player::get_keybindings(), Queue::get_keybindings()].concat();

                rect.render_widget(draw_keys(keybindings, color, highlight_color), chunks[3]);
            }) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to render frame: {}", err);
                }
            }

            loop {
                if let Some(code) = poll_key() {
                    parent.process_input(handler, code);
                    self.process_input(handler, code);
                }

                if !self.state.is_paused {
                    break;
                } else {
                    trigger(AudioEvent::ResetTick, handler, self);
                }
            }
            thread::sleep(Duration::from_millis(interval.into()));
        }
    }
}

impl Instance for Player {
    fn get_keybindings() -> Vec<Key> {
        get_player_keybindings()
    }

    fn process_input<B: Backend>(&mut self, handler: &mut EventHandler<B>, code: KeyCode) {
        process_player_input(handler, self, code)
    }
}

impl Player {
    pub fn new(path: &str, volume: f32, speed: f32) -> Option<Player> {
        let source_data = match SourceData::new(path) {
            Some(source_data) => source_data,
            None => return None,
        };

        let duration = source_data.duration;

        let (sink, _stream) = match init_audio_handler() {
            Some(handler_data) => handler_data,
            None => return None,
        };

        sink.set_volume(volume);
        sink.set_speed(speed);

        Some(Player {
            sink,
            _stream,
            source_data,
            state: AudioState::new(duration),
        })
    }
}
