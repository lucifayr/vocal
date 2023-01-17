pub struct SelectionKeybindings {
    quit: String,
    play: String,
    add_and_play: String,
    up: String,
    down: String,
    add_to_queue: String,
    remove_from_queue: String,
}

impl std::default::Default for SelectionKeybindings {
    fn default() -> Self {
        SelectionKeybindings {
            quit: "q".to_owned(),
            play: "p".to_owned(),
            add_and_play: "Enter".to_owned(),
            up: "Up/k".to_owned(),
            down: "Down/j".to_owned(),
            add_to_queue: "Right/l".to_owned(),
            remove_from_queue: "Left/h".to_owned(),
        }
    }
}

impl SelectionKeybindings {
    pub fn get_keybindings(&self) -> [(&str, &str); 7] {
        [(self.quit.as_str(), "quit")
        ,(self.play.as_str(), "play")
        ,(self.add_and_play.as_str(), "add_and_play")
        ,(self.up.as_str(), "up")
        ,(self.down.as_str(), "down")
        ,(self.add_to_queue.as_str(), "add_to_queue")
        ,(self.remove_from_queue.as_str(), "remove_from_queue")]
    }
}
