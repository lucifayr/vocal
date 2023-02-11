use crossterm::event::KeyCode;

#[derive(Debug, Clone)]
pub struct Key {
    pub key: KeyCode,
    pub hint: &'static str,
}
