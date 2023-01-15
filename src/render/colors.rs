use tui::style::Color;

pub fn get_color(name: &str) -> Color {
    match name {
        "black" | "Black" => Color::Black,
        "red" | "Red" => Color::Red,
        "green" | "Green" => Color::Green,
        "yellow" | "Yellow" => Color::Yellow,
        "blue" | "Blue" => Color::Blue,
        "magenta" | "Magenta" => Color::Magenta,
        "cyan" | "Cyan" => Color::Cyan,
        "gray" | "Gray" => Color::Gray,
        "lightred" | "LightRed" => Color::LightRed,
        "lightgreen" | "LightGreen" => Color::LightGreen,
        "lightyellow" | "LightYellow" => Color::LightYellow,
        "lightblue" | "LightBlue" => Color::LightBlue,
        "lightmagenta" | "LightMagenta" => Color::LightMagenta,
        "lightcyan" | "LightCyan" => Color::LightCyan,
        "white" | "White" => Color::White,
        _ => Color::Blue,
    }
}
