use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::input::key::Key;

pub fn draw_keys<'a>(content: Vec<Key>, color: Color, highlight_color: Color) -> Paragraph<'a> {
    let text: String = content
        .iter()
        .map(|item| format!("  {}: {}  ", item.key(), item.hint()))
        .collect();

    Paragraph::new(Text::styled(text, Style::default().fg(highlight_color)))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Keys")
                .style(Style::default().fg(color))
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true })
}
