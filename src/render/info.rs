use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::Paragraph,
};

pub fn draw_info(
    path: &str,
    volume: u8,
    is_muted: bool,
    speed: u8,
    duration_secs: f64,
    passed_time: f64,
    color: Color,
) -> Paragraph {
    let name = get_filename_from_path(path).unwrap_or("???");

    let mute_symbol = if is_muted { "✗" } else { "" };

    let duration_hours = (duration_secs / 3600_f64).round();
    let duration_min = (duration_secs / 60_f64).round();
    let duration_seconds = (duration_secs.round() as i64) % 60;

    let passed_hours = (passed_time / 3600_f64).round();
    let passed_min = (passed_time / 60_f64).round();
    let passed_seconds = (passed_time.round() as i64) % 60;

    let speed_multiplier = speed as f32 / 100_f32;

    Paragraph::new(Text::styled(
        format!(
            "Playing: {}\n\nVolume: {}% {}\nPlayback Speed: {:0.2}X\nDuration: {:02}:{:02}:{:02}\nPlayed: {:02}:{:02}:{:02}\n",
            name,
            volume,
            mute_symbol,
            speed_multiplier,
            duration_hours,
            duration_min,
            duration_seconds,
            passed_hours,
            passed_min,
            passed_seconds

        ),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
}

pub fn draw_info_no_audio<'a>(path_to_audio_directory: String, color: Color) -> Paragraph<'a> {
    Paragraph::new(Text::styled(
        format!(
            "Add some files into {} or pass some paths to the vocal --load command",
            path_to_audio_directory
        ),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}

pub fn get_filename_from_path(path: &str) -> Option<&str> {
    path.split('/').last()?.split('.').next()
}

#[test]
fn test_get_filename_from_path() {
    assert_eq!(get_filename_from_path(""), Some(""));
    assert_eq!(get_filename_from_path("file"), Some("file"));
    assert_eq!(get_filename_from_path("file.mp3"), Some("file"));
    assert_eq!(get_filename_from_path("file.mp3.exe"), Some("file"));
    assert_eq!(get_filename_from_path("folder/file"), Some("file"));
    assert_eq!(get_filename_from_path("folder/file.mp3"), Some("file"));
    assert_eq!(
        get_filename_from_path("folder/folder/file.mp3"),
        Some("file")
    );
}
