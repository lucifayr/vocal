use rodio::{source::SineWave, Decoder, OutputStream, Source};
use std::{io::BufReader, thread, time::Duration};

use crate::unicode::render::render_loading_bar;

mod unicode;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = std::fs::File::open("mock_audio/rick.mp3").unwrap();
    let source = Decoder::new(file).unwrap();

    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(500));

    // source.for_each(|f| println!("{:?}", f));
    // for i in 0..1000 {
    //     let bar = render_loading_bar(i as f32, 0.0, 1000.0, 100, unicode::colors::Color::Blue);
    //     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //     println!("{bar}");
    //     thread::sleep(Duration::from_millis(10));
    // }
}
