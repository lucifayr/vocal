use rand::Rng;
use tui::style::Color;

pub fn get_color(is_rainbow: bool) -> Color {
    let mut rng = rand::thread_rng();

    if is_rainbow {
        match rng.gen_range(1..7) {
            x if x == 1 => Color::Blue,
            x if x == 2 => Color::Yellow,
            x if x == 3 => Color::Red,
            x if x == 4 => Color::Green,
            x if x == 5 => Color::Magenta,
            _ => Color::White,
        }
    } else {
        Color::Blue
    }
}

