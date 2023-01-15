use tui::style::Color;

use crate::render::colors::{get_color, get_highlight_color};

pub struct RuntimeOptions {
    pub is_muted: bool,
    pub volume: u8,
    pub speed: u8,
    pub volume_decimal: f32,
    pub speed_decimal: f32,
    pub color: Color,
    pub highlight_color: Color,
}

impl RuntimeOptions {
    pub fn new(volume: u8, speed: u8) -> RuntimeOptions {
        RuntimeOptions {
            is_muted: false,
            volume,
            speed,
            volume_decimal: volume as f32 / 100_f32,
            speed_decimal: speed as f32 / 100_f32,
            color: get_color(),
            highlight_color: get_highlight_color(),
        }
    }
}
