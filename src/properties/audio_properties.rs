use std::time::{Duration, Instant};

pub struct AudioOptions {
    pub is_paused: bool,
    pub duration: Duration,
    pub time_since_last_tick: Instant,
    pub passed_time: f64,
}

impl AudioOptions {
    pub fn new(duration: Duration) -> AudioOptions {
        AudioOptions {
            is_paused: false,
            duration,
            time_since_last_tick: Instant::now(),
            passed_time: 0_f64,
        }
    }
}
