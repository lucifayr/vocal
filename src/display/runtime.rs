use std::time::{Duration, Instant};

pub struct RuntimeOptions {
    pub is_muted: bool,
    pub is_paused: bool,
    pub volume: u8,
    pub speed: u8,
    pub volume_decimal: f32,
    pub speed_decimal: f32,
    pub duration: Duration,
    pub duration_secs: f64,
    pub time_since_last_pause_tick: Instant,
    pub paused_time: f64,
}

pub fn init_runtime_options(volume: u8, speed: u8, duration: Duration) -> RuntimeOptions {
    let volume_decimal = volume as f32 / 100.0;
    let speed_decimal = speed as f32 / 100.0;
    let duration_secs = duration.as_secs_f64() / speed_decimal as f64;

    RuntimeOptions {
        is_muted: false,
        is_paused: false,
        volume,
        speed,
        volume_decimal,
        speed_decimal,
        duration,
        duration_secs,
        time_since_last_pause_tick: Instant::now(),
        paused_time: 0.0,
    }
}
