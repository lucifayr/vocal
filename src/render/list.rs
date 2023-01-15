use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub fn draw_list<'a>(items: Vec<ListItem<'a>>, color: Color, highlight_color: Color) -> List<'a> {
    List::new(items)
        .block(Block::default().title("Audio").borders(Borders::ALL))
        .style(Style::default().fg(color))
        .highlight_style(Style::default().fg(highlight_color))
}
