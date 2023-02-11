use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct AudioState {
    pub is_paused: bool,
    pub duration: Duration,
    pub time_since_last_tick: Instant,
    pub passed_time: f64,
    pub progress: f64,
}

impl AudioState {
    pub fn new(duration: Duration) -> AudioState {
        AudioState {
            is_paused: false,
            duration,
            time_since_last_tick: Instant::now(),
            passed_time: 0_f64,
            progress: 0_f64,
        }
    }
}
