use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub color: String,
    pub highlight_color: String,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            color: "blue".to_owned(),
            highlight_color: "magenta".to_owned(),
        }
    }
}
