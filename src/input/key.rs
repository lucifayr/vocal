#[derive(Debug, Clone)]
pub struct Key {
    key: String,
    hint: String,
}

impl Key {
    pub fn new(key: &str, hint: &str) -> Self {
        Key {
            key: key.to_owned(),
            hint: hint.to_owned(),
        }
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn hint(&self) -> &str {
        self.hint.as_str()
    }
}
