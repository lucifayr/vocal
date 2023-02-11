pub struct RuntimeState {
    pub is_muted: bool,
    pub volume: u8,
    pub speed: u8,
}

impl RuntimeState {
    pub fn new(volume: u8, speed: u8) -> Self {
        Self {
            is_muted: false,
            volume,
            speed,
        }
    }

    pub fn get_volume_decimal(&self) -> f32 {
        self.volume as f32 / 100_f32
    }

    pub fn get_speed_decimal(&self) -> f32 {
        self.speed as f32 / 100_f32
    }
}
