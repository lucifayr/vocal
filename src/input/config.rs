use std::env;

use serde_derive::{Deserialize, Serialize};
use tui::style::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub color: String,
    pub highlight_color: String,
    pub audio_directory: String,
}

impl std::default::Default for Config {
    fn default() -> Self {
        let home_directory = match env::var("HOME") {
            Ok(path) => path,
            Err(_) => "./vocal".to_owned(),
        };

        Self {
            color: "blue".to_owned(),
            highlight_color: "magenta".to_owned(),
            audio_directory: format!("{home_directory}/vocal"),
        }
    }
}

impl Config {
    pub fn get_color(&self) -> Color {
        match self.color.as_str() {
            "black" | "Black" => Color::Black,
            "red" | "Red" => Color::Red,
            "green" | "Green" => Color::Green,
            "yellow" | "Yellow" => Color::Yellow,
            "blue" | "Blue" => Color::Blue,
            "magenta" | "Magenta" => Color::Magenta,
            "cyan" | "Cyan" => Color::Cyan,
            "gray" | "Gray" => Color::Gray,
            "lightred" | "LightRed" => Color::LightRed,
            "lightgreen" | "LightGreen" => Color::LightGreen,
            "lightyellow" | "LightYellow" => Color::LightYellow,
            "lightblue" | "LightBlue" => Color::LightBlue,
            "lightmagenta" | "LightMagenta" => Color::LightMagenta,
            "lightcyan" | "LightCyan" => Color::LightCyan,
            "white" | "White" => Color::White,
            _ => Color::Blue,
        }
    }

    pub fn get_highlight_color(&self) -> Color {
        match self.highlight_color.as_str() {
            "black" | "Black" => Color::Black,
            "red" | "Red" => Color::Red,
            "green" | "Green" => Color::Green,
            "yellow" | "Yellow" => Color::Yellow,
            "blue" | "Blue" => Color::Blue,
            "magenta" | "Magenta" => Color::Magenta,
            "cyan" | "Cyan" => Color::Cyan,
            "gray" | "Gray" => Color::Gray,
            "lightred" | "LightRed" => Color::LightRed,
            "lightgreen" | "LightGreen" => Color::LightGreen,
            "lightyellow" | "LightYellow" => Color::LightYellow,
            "lightblue" | "LightBlue" => Color::LightBlue,
            "lightmagenta" | "LightMagenta" => Color::LightMagenta,
            "lightcyan" | "LightCyan" => Color::LightCyan,
            "white" | "White" => Color::White,
            _ => Color::Magenta,
        }
    }
}
