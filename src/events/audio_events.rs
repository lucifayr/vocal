// use std::time::Instant;

// use rodio::Sink;
// use tui::backend::Backend;

// use crate::state::audio_state::AudioState;

// use super::handler::{Event, EventHandler};

// pub enum AudioEvent {
//     StartAudio,
//     EndAudio,
//     PauseAudio,
//     MuteAudio,
//     ResetSpeed,
//     VolumeUp,
//     VolumeDown,
//     SpeedUp,
//     SpeedDown,
//     Tick,
//     ResetTick,
// }

// trait AudioActions {
//     fn pause(&mut self);
//     fn mute(&mut self);
//     fn reset_speed(&mut self);
//     fn volume_up(&mut self);
//     fn volume_down(&mut self);
//     fn speed_up(&mut self);
//     fn speed_down(&mut self);
//     fn tick(&mut self);
//     fn reset_tick(&mut self);
// }

// fn pause(state: &mut AudioState, sink: &Sink) {
//     state.is_paused = !state.is_paused;
//     if state.is_paused {
//         sink.pause();
//     } else {
//         sink.play();
//     }
// }

// impl<B: Backend> AudioActions for EventHandler<B> {
//     fn pause(&mut self) {
//         if let Some(instance) = self.audio_instance.as_mut() {
//             instance.a_state.is_paused = !instance.a_state.is_paused;
//             if instance.a_state.is_paused {
//                 self.sink.pause();
//             } else {
//                 self.sink.play();
//             }
//         }
//     }

//     fn mute(&mut self) {
//         if !self.r_state.is_muted {
//             self.sink.set_volume(0.0);
//             self.r_state.is_muted = true;
//         } else {
//             self.sink.set_volume(self.r_state.volume_decimal);
//             self.r_state.is_muted = false;
//         }
//     }

//     fn reset_speed(&mut self) {
//         self.r_state.speed = 100;
//         self.r_state.speed_decimal = self.r_state.speed as f32 / 100.0;
//         self.sink.set_speed(self.r_state.speed_decimal);
//     }

//     fn volume_up(&mut self) {
//         self.r_state.volume /= 10;
//         self.r_state.volume *= 10;

//         if self.r_state.volume < 100 {
//             self.r_state.volume += 10;
//             self.r_state.volume_decimal = self.r_state.volume as f32 / 100.0;
//             if !self.r_state.is_muted {
//                 self.sink.set_volume(self.r_state.volume_decimal);
//             }
//         }
//     }

//     fn volume_down(&mut self) {
//         self.r_state.volume /= 10;
//         self.r_state.volume *= 10;

//         if self.r_state.volume > 0 {
//             self.r_state.volume -= 10;
//             self.r_state.volume_decimal = self.r_state.volume as f32 / 100.0;
//             if !self.r_state.is_muted {
//                 self.sink.set_volume(self.r_state.volume_decimal);
//             }
//         }
//     }

//     fn speed_up(&mut self) {
//         self.r_state.speed /= 10;
//         self.r_state.speed *= 10;

//         if self.r_state.speed < 200 {
//             self.r_state.speed += 10;
//             self.r_state.speed_decimal = self.r_state.speed as f32 / 100.0;
//             self.sink.set_speed(self.r_state.speed_decimal);
//         }
//     }

//     fn speed_down(&mut self) {
//         self.r_state.speed /= 10;
//         self.r_state.speed *= 10;

//         if self.r_state.speed > 10 {
//             self.r_state.speed -= 10;
//             self.r_state.speed_decimal = self.r_state.speed as f32 / 100.0;
//             self.sink.set_speed(self.r_state.speed_decimal);
//         }
//     }

//     fn tick(&mut self) {
//         if let Some(instance) = self.audio_instance.as_mut() {
//             instance.a_state.passed_time += instance
//                 .a_state
//                 .time_since_last_tick
//                 .elapsed()
//                 .as_secs_f64()
//                 * self.r_state.speed_decimal as f64;
//             instance.a_state.time_since_last_tick = Instant::now();

//             instance.a_state.progress =
//                 instance.a_state.passed_time / instance.a_state.duration.as_secs_f64();
//         }
//     }

//     fn reset_tick(&mut self) {
//         if let Some(instance) = self.audio_instance.as_mut() {
//             instance.a_state.time_since_last_tick = Instant::now();
//         }
//     }
// }

// impl Event for AudioEvent {
//     fn trigger<B: Backend>(&self, handler: &mut EventHandler<B>) {
//         match self {
//             AudioEvent::StartAudio => {}
//             AudioEvent::EndAudio => {}
//             AudioEvent::PauseAudio => handler.pause(),
//             AudioEvent::MuteAudio => handler.mute(),
//             AudioEvent::ResetSpeed => handler.reset_speed(),
//             AudioEvent::VolumeUp => handler.volume_up(),
//             AudioEvent::VolumeDown => handler.volume_down(),
//             AudioEvent::SpeedUp => handler.speed_up(),
//             AudioEvent::SpeedDown => handler.speed_down(),
//             AudioEvent::Tick => handler.tick(),
//             AudioEvent::ResetTick => handler.reset_tick(),
//         }
//     }
// }
