use std::{fs::File, thread, time::Duration};

use rodio::{Decoder, Source};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    audio::source_data::SourceData,
    events::{audio_events::AudioEvent, handler::EventHandler},
    input::{audio_keybindings::AudioKeybindings},
    properties::audio_properties::AudioOptions,
    render::{
        bar::draw_bar,
        chart::{create_data_from_samples, draw_chart},
        info::draw_info,
        keybindings::draw_keys,
    },
};

#[derive(Debug, Clone)]
pub struct AudioInstance {
    pub audio_options: AudioOptions,
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

    pub fn play_queue<B: Backend>(content: Vec<String>, handler: &mut EventHandler<B>) {
        handler.trigger(AudioEvent::StartQueue);

        match handler.terminal.clear() {
            Ok(_) => {}
            Err(_) => println!("Failed to clear terminal"),
        }

        for audio in content {
            AudioInstance::start_instance(audio, handler)
        }

        handler.trigger(AudioEvent::EndQueue);
    }

    pub fn start_instance<B: Backend>(path: String, handler: &mut EventHandler<B>) {
        if let Some(instance) = AudioInstance::new(path.as_str()) {
            let source = match SourceData::get_source(path.as_str()) {
                Some(source) => source,
                None => return,
            };

            handler.audio_instance = Some(instance);
            match AudioInstance::play_audio(source, handler) {
                Ok(_) => {}
                Err(err) => println!("{err}"),
            };
        };
    }

    pub fn play_audio<'a, B: Backend>(
        source: Decoder<File>,
        handler: &'a mut EventHandler<B>,
    ) -> Result<(), &'a str> {
        handler.trigger(AudioEvent::StartAudio);
        let terminal_size = match handler.terminal.size() {
            Ok(size) => size,
            Err(_) => return Err("Failed to get terminal size"),
        };

        let keybindings = AudioKeybindings::default();

        let interval = 16;
        let sample_rate = source.sample_rate();
        let step = (sample_rate * interval) as f32 / 1000.0;

        handler.sink.append(source);
        loop {
            handler.trigger(AudioEvent::Tick);

            let instance = handler
                .audio_instance
                .as_ref()
                .expect("Audio instance should exist")
                .clone();

            let progress = instance.audio_options.progress;

            if progress > 1.0 {
                handler.trigger(AudioEvent::EndAudio);
                return Ok(());
            }

            let start = (progress * instance.source_data.samples.len() as f64) as usize;
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
                    instance.source_data.samples.clone(),
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
                            handler.config.get_color(),
                        ),
                        chunks[0],
                    );
                }

                rect.render_widget(
                    draw_info(
                        instance.path.as_str(),
                        handler.runtime_options.volume,
                        handler.runtime_options.is_muted,
                        handler.runtime_options.speed,
                        instance.audio_options.duration.as_secs_f64(),
                        instance.audio_options.passed_time,
                        handler.config.get_highlight_color(),
                    ),
                    chunks[1],
                );
                rect.render_widget(draw_bar(progress, handler.config.get_color()), chunks[2]);

                rect.render_widget(
                    draw_keys(
                        keybindings.get_keybindings(),
                        handler.config.get_color(),
                        handler.config.get_highlight_color(),
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
                keybindings.pull_input(handler);
                if !handler
                    .audio_instance
                    .as_ref()
                    .expect("Audio instance should exist")
                    .audio_options
                    .is_paused
                {
                    break;
                } else {
                    handler.trigger(AudioEvent::ResetTick);
                }
            }
            thread::sleep(Duration::from_millis(interval.into()));
        }
    }
}
