pub struct RuntimeOptions {
    pub is_muted: bool,
    pub volume: u8,
    pub speed: u8,
    pub volume_decimal: f32,
    pub speed_decimal: f32,
}

impl RuntimeOptions {
    pub fn new(volume: u8, speed: u8) -> RuntimeOptions {
        RuntimeOptions {
            is_muted: false,
            volume,
            speed,
            volume_decimal: volume as f32 / 100_f32,
            speed_decimal: speed as f32 / 100_f32,
        }
    }
}
