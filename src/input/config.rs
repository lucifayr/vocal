use std::{
    env,
    fs::{create_dir_all, read_dir},
};

use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use tui::style::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub color: String,
    pub highlight_color: String,
    pub audio_directory: String,
    pub log_directory: String,
    pub log_file_prefix: String,
    pub starting_volume: u8,
    pub starting_speed: u8,
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
            audio_directory: format!("{home_directory}/vocal/audio"),
            log_directory: format!("{home_directory}/vocal/logs"),
            log_file_prefix: "vocal".to_owned(),
            starting_volume: 50,
            starting_speed: 100,
        }
    }
}

impl Config {
    pub fn get_audio_directory_content(path: &str) -> Result<Vec<String>, &'static str> {
        match create_dir_all(path) {
            Ok(_) => match read_dir(path) {
                Ok(paths) => Ok(paths
                    .map(|path| match path {
                        Ok(path) => path.path().display().to_string(),
                        Err(_) => "???".to_owned(),
                    })
                    .collect()),
                Err(_) => Err("Failed to open default audio directory"),
            },
            Err(_) => Err("Failed to create default audio directory"),
        }
    }

    pub fn get_color(&self) -> Color {
        match self.color.as_str() {
            "rainbow" | "Rainbow" => get_random_color(),
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

fn get_random_color() -> Color {
    let r = rand::thread_rng().gen_range(0..15);
    match r {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::Gray,
        8 => Color::LightRed,
        9 => Color::LightGreen,
        10 => Color::LightYellow,
        11 => Color::LightBlue,
        12 => Color::LightMagenta,
        13 => Color::LightCyan,
        _ => Color::White,
    }
}
