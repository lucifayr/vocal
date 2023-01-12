use termion::cursor;

use crate::{display::terminal::TerminalData, unicode::colors::Color};

pub fn render_bar_graph(
    samples: Vec<f32>,
    terminal_data: &TerminalData,
    bar_count: usize,
    color: Color,
) -> String {
    let width = terminal_data.x / bar_count as u16;
    let height = terminal_data.y / 2;
    let max_value = 1.0;

    let mut graph = "".to_owned();
    for (i, value) in samples.iter().enumerate() {
        let bar_height = (value / max_value) * height as f32;
        for w in 0..width {
            let x_position = i as u16 * width + (w + 1);
            for h in 0..bar_height as u16 {
                let y_position = match h {
                    h if h > height => 0,
                    _ => height - h,
                };
                if x_position <= terminal_data.x || y_position <= terminal_data.y {
                    graph += format!(
                        "\x1b[3{color}m{}{}\x1b[m",
                        cursor::Goto(x_position, y_position),
                        '█',
                    )
                    .as_str();
                }
            }
        }
    }

    graph
}
