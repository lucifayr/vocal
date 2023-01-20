#[derive(Debug, Clone)]
pub struct Key {
    key: char,
    hint_key: String,
    hint: String,
}

impl Key {
    pub fn new(key: char, hint_key: &str, hint: &str) -> Self {
        Key {
            key,
            hint_key: hint_key.to_owned(),
            hint: hint.to_owned(),
        }
    }

    pub fn key(&self) -> char {
        self.key
    }

    pub fn hint_key(&self) -> &str {
        self.hint_key.as_str()
    }

    pub fn hint(&self) -> &str {
        self.hint.as_str()
    }
}
