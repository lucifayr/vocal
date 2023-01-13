use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::Paragraph,
};

use super::text::get_filename_from_path;

pub fn draw_info(path: &str, volume: u8, is_muted: bool, speed: u8, color: Color) -> Paragraph {
    let name = match get_filename_from_path(path) {
        Some(name) => name,
        None => "???",
    };

    let mute_symbol = if is_muted { "✗" } else { "" };

    Paragraph::new(Text::styled(
        format!(
            "Playing: {}\n\nVolume: {} {}\nPlayback Speed: {}",
            name, volume, mute_symbol, speed
        ),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
}
