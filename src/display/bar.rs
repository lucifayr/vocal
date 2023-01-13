use tui::{
    style::{Color, Style},
    widgets::{Block, Gauge},
};

pub fn draw_bar<'a>(progress: f32, color: Color) -> Gauge<'a> {
    let bar_progress = (progress * 100.0) as u16;
    Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(color).bg(Color::Reset))
        .percent(bar_progress)
        .label("")
}
