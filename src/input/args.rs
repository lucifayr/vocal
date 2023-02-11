use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to audio files to play
    // #[arg(short, long, num_args(1..), group = "input")]
    // pub play: Option<Vec<String>>,

    /// Path to audio files to load
    #[arg(short, long, num_args(1..), group = "input")]
    pub load: Option<Vec<String>>,
}
