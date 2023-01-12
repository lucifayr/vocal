use std::{fs::File, time::Duration};

use rodio::Decoder;

pub struct SourceData {
    pub source: Decoder<File>,
    pub duration: Duration,
    pub path: String,
}
