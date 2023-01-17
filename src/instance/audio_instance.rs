use std::{
    fs::File,
    thread,
    time::{Duration, Instant},
};

use rodio::{Decoder, Sink, Source};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::{
    audio::source_data::SourceData,
    properties::{audio_properties::AudioOptions, runtime_properties::RuntimeOptions},
    render::{
        bar::draw_bar,
        chart::{create_data_from_samples, draw_chart},
        info::draw_info,
        keys::draw_keys,
    },
    user_input::{config::Config, input::pull_input_while_playing, keybindings::AudioKeybindings},
};

pub struct AudioInstance {
    audio_options: AudioOptions,
    source_data: SourceData,
    path: String,
}

impl AudioInstance {
    pub fn new(path: &str) -> Option<AudioInstance> {
        let source_data = match SourceData::new(path) {
            Some(source_data) => source_data,
            None => return None,
        };

        let duration = source_data.duration;

        Some(AudioInstance {
            source_data,
            audio_options: AudioOptions::new(duration),
            path: path.to_owned(),
        })
    }

    pub fn start_instance<B: Backend>(
        path: String,
        sink: &mut Sink,
        runtime_options: &mut RuntimeOptions,
        config: &Config,
        terminal: &mut Terminal<B>,
    ) {
        if let Some(mut instance) = AudioInstance::new(path.as_str()) {
            let source = match SourceData::get_source(path.as_str()) {
                Some(source) => source,
                None => return,
            };

            match instance.play_audio(sink, source, runtime_options, config, terminal) {
                Ok(_) => {}
                Err(err) => println!("{err}"),
            };
        };
    }

    pub fn play_audio<B: Backend>(
        &mut self,
        sink: &mut Sink,
        source: Decoder<File>,
        runtime_options: &mut RuntimeOptions,
        config: &Config,
        terminal: &mut Terminal<B>,
    ) -> Result<(), &str> {
        match terminal.clear() {
            Ok(_) => {}
            Err(_) => return Err("Failed to clear terminal"),
        }

        let terminal_size = match terminal.size() {
            Ok(size) => size,
            Err(_) => return Err("Failed to get terminal size"),
        };

        let interval = 16;
        let sample_rate = source.sample_rate();
        let step = (sample_rate * interval) as f32 / 1000.0;

        sink.append(source);

        loop {
            self.audio_options.passed_time += self
                .audio_options
                .time_since_last_tick
                .elapsed()
                .as_secs_f64()
                * runtime_options.speed_decimal as f64;

            self.audio_options.time_since_last_tick = Instant::now();

            let progress =
                self.audio_options.passed_time / self.audio_options.duration.as_secs_f64();
            if progress > 1.0 {
                return Ok(());
            }

            let start = (progress * self.source_data.samples.len() as f64) as usize;
            let bar_count = (terminal_size.width / 2) as usize;

            match terminal.draw(|rect| {
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
                        draw_chart(data.as_slice(), max * multiplier as u64, config.get_color()),
                        chunks[0],
                    );
                }

                rect.render_widget(
                    draw_info(
                        self.path.as_str(),
                        runtime_options.volume,
                        runtime_options.is_muted,
                        runtime_options.speed,
                        self.audio_options.duration.as_secs_f64(),
                        self.audio_options.passed_time,
                        config.get_color(),
                    ),
                    chunks[1],
                );
                rect.render_widget(draw_bar(progress, config.get_color()), chunks[2]);

                rect.render_widget(
                    draw_keys(
                        AudioKeybindings::default().get_keybindings().as_slice(),
                        config.get_color(),
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
                pull_input_while_playing(sink, runtime_options, &mut self.audio_options);
                if !self.audio_options.is_paused {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(interval.into()));
        }
    }
}
