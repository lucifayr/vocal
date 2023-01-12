pub struct TerminalData {
    pub x: u16,
    pub y: u16,
}

pub fn get_terminal_data() -> Option<TerminalData> {
    let (x, y) = match termion::terminal_size() {
        Ok(size) => size,
        Err(_) => return None,
    };

    Some(TerminalData { x, y })
}
