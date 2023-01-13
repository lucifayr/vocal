use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::Paragraph,
};

use super::text::get_filename_from_path;

pub fn draw_info(
    path: &str,
    volume: u8,
    is_muted: bool,
    speed: u8,
    duration_secs: f64,
    passed_time: f64,
    color: Color,
) -> Paragraph {
    let name = match get_filename_from_path(path) {
        Some(name) => name,
        None => "???",
    };

    let mute_symbol = if is_muted { "✗" } else { "" };

    Paragraph::new(Text::styled(
        format!(
            "Playing: {}\n\nVolume: {} {}\nPlayback Speed: {}\nDuration: {}s\nPlayed: {}s\n",
            name,
            volume,
            mute_symbol,
            speed,
            duration_secs.round(),
            passed_time.round()
        ),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
}
