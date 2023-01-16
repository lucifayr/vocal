use tui::{
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
};

pub fn draw_chart<'a>(data: &'a [(&'a str, u64)], max: u64, color: Color) -> BarChart<'a> {
    BarChart::default()
        .block(Block::default().borders(Borders::BOTTOM))
        .bar_width(3)
        .bar_style(Style::default().fg(color).bg(Color::Reset))
        .bar_gap(1)
        .value_style(Style::default().fg(color).bg(color))
        .label_style(Style::default())
        .data(data)
        .max(max)
}

pub fn create_data_from_samples<'a>(
    samples: Vec<f32>,
    start: usize,
    step: usize,
    bar_count: usize,
    max: u64,
    multiplier: f32,
) -> Option<Vec<(&'a str, u64)>> {
    let reduced_samples = match reduce_sample_to_slice(samples, start, step as usize, bar_count) {
        Ok(samples) => samples,
        Err(_) => return None,
    };

    Some(
        reduced_samples
            .iter()
            .map(|sample| ("", (max as f32 * multiplier * sample) as u64))
            .collect::<Vec<(&str, u64)>>(),
    )
}

fn reduce_sample_to_slice(
    samples: Vec<f32>,
    start: usize,
    step: usize,
    bar_count: usize,
) -> Result<Vec<f32>, ()> {
    let sample_slice: Vec<f32> = samples
        .clone()
        .iter()
        .skip(start)
        .take(step)
        .map(|s| ((*s + 1_f32) / 2_f32))
        .collect();

    let chunk_size = match sample_slice.len() / bar_count {
        x if x != 0 => x,
        _ => return Err(()),
    };

    Ok(sample_slice
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().sum::<f32>() / chunk_size as f32)
        .take(bar_count)
        .collect())
}
