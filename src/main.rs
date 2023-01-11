use std::{thread, time::Duration};

use crate::unicode::render::render_loading_bar;

mod unicode;

fn main() {
    for i in 0..1000 {
        let bar = render_loading_bar(i as f32, 0.0, 1000.0, 100, unicode::colors::Color::Blue);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{bar}");
        thread::sleep(Duration::from_millis(10));
    }
}
