use tui::{
    style::{Color, Style},
    widgets::{Block, List, ListItem},
};

pub fn draw_list<'a>(items: Vec<ListItem<'a>>, color: Color, highlight_color: Color) -> List<'a> {
    List::new(items)
        .block(Block::default().title(""))
        .style(Style::default().fg(color))
        .highlight_style(Style::default().fg(color).bg(highlight_color))
}
