use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_footer<'a>(
    text: String,
    show_border: bool,
    color: Color,
    highlight_color: Color,
) -> Paragraph<'a> {
    let block = if show_border {
        Block::default()
            .title("Hotkeys")
            .style(Style::default().fg(color))
            .borders(Borders::ALL)
    } else {
        Block::default()
    };

    Paragraph::new(Text::styled(text, Style::default().fg(highlight_color)))
        .alignment(Alignment::Center)
        .block(block)
        .wrap(Wrap { trim: true })
}
