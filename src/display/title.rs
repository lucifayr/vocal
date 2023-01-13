use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::Paragraph,
};

use super::text::get_filename_from_path;

pub fn draw_title(path: &str, color: Color) -> Paragraph {
    let name = match get_filename_from_path(path) {
        Some(name) => name,
        None => "???",
    };

    Paragraph::new(Text::styled(
        format!("Playing: {}\n\n", name),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
}
