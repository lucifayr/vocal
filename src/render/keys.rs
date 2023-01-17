use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_keys<'a>(content: &'a [(&'a str, &'a str)], color: Color) -> Paragraph {
    let text: String = content
        .iter()
        .map(|item| {
            let (key, effect) = item;
            format!("  {key}: {effect}  ")
        })
        .collect();

    Paragraph::new(Text::styled(text, Style::default().fg(color)))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Keys")
                .style(Style::default().fg(color))
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true })
}
