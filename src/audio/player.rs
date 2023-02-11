use std::{thread, time::Duration};

use rodio::Source;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    audio::source_data::SourceData,
    events::handler::EventHandler,
    input::playback_keybindings::PlaybackKeybindings,
    instance::queue::Queue,
    render::{
        bar::draw_bar,
        chart::{create_data_from_samples, draw_chart},
        info::draw_info,
        keybindings::draw_keys,
    },
    state::audio_state::AudioState,
};

pub struct Player {
    state: AudioState,
    interupted: bool,
    source_data: SourceData,
    path: String,
}

impl Player {
    pub fn new(path: &str) -> Option<Player> {
        let source_data = match SourceData::new(path) {
            Some(source_data) => source_data,
            None => return None,
        };

        let duration = source_data.duration;

        Some(Player {
            source_data,
            state: AudioState::new(duration),
            path: path.to_owned(),
            interupted: false,
        })
    }

    pub fn start_audio<B: Backend>(path: String, handler: &mut EventHandler<B, Queue>) {
        if let Some(mut audio) = Player::new(path.as_str()) {
            match audio.play(handler) {
                Ok(_) => {}
                Err(err) => println!("{err}"),
            };
        };
    }

    pub fn play<B: Backend>(&mut self, handler: &mut EventHandler<B, Queue>) -> Result<(), &str> {
        // handler.trigger(AudioEvent::StartAudio);
        let terminal_size = match handler.get_terminal_size() {
            Ok(size) => size,
            Err(_) => return Err("Failed to get terminal size"),
        };

        let keybindings = PlaybackKeybindings::default();

        let interval = 16;
        let source = self.source_data.source;
        let sample_rate = source.sample_rate();
        let step = (sample_rate * interval) as f32 / 1000.0;

        handler.instance.sink.append(source);
        loop {
            if handler.instance.interupted {
                // handler.trigger(AudioEvent::EndAudio);
                return Ok(());
            }

            // handler.trigger(AudioEvent::Tick);

            let progress = self.state.progress;

            if progress > 1.0 {
                // handler.trigger(AudioEvent::EndAudio);
                return Ok(());
            }

            let start = (progress * self.source_data.samples.len() as f64) as usize;
            let bar_count = (terminal_size.width / 2) as usize;

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
                        draw_chart(
                            data.as_slice(),
                            max * multiplier as u64,
                            handler.get_config().get_color(),
                        ),
                        chunks[0],
                    );
                }

                rect.render_widget(
                    draw_info(
                        self.path.as_str(),
                        handler.get_state().volume,
                        handler.get_state().is_muted,
                        handler.get_state().speed,
                        self.state.duration.as_secs_f64(),
                        self.state.passed_time,
                        handler.get_config().get_highlight_color(),
                    ),
                    chunks[1],
                );
                rect.render_widget(
                    draw_bar(progress, handler.get_config().get_color()),
                    chunks[2],
                );

                rect.render_widget(
                    draw_keys(
                        keybindings.get_keybindings(),
                        handler.get_config().get_color(),
                        handler.get_config().get_highlight_color(),
                    ),
                    chunks[3],
                );
            }) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to render frame: {}", err);
                }
            }

            loop {
                // keybindings.pull_input(handler);
                if !self.state.is_paused {
                    break;
                } else {
                    // handler.trigger(AudioEvent::ResetTick);
                }
            }
            thread::sleep(Duration::from_millis(interval.into()));
        }
    }
}
