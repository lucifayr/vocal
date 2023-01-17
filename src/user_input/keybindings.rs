pub struct SelectionKeybindings {
    quit: String,
    play: String,
    add_and_play: String,
    up: String,
    down: String,
    add_to_queue: String,
    remove_from_queue: String,
}

pub struct AudioKeybindings {
    quit: String,
    pause: String,
    mute: String,
    volume_up: String,
    volume_down: String,
    speed_up: String,
    speed_down: String,
    reset_speed: String,
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

impl std::default::Default for AudioKeybindings {
    fn default() -> Self {
        AudioKeybindings {
            quit: "q".to_owned(),
            pause: "Space".to_owned(),
            mute: "m".to_owned(),
            volume_up: "Up/k".to_owned(),
            volume_down: "Down/j".to_owned(),
            speed_up: "Rigth/l".to_owned(),
            speed_down: "Left/h".to_owned(),
            reset_speed: "r".to_owned(),
        }
    }
}

impl SelectionKeybindings {
    pub fn get_keybindings(&self) -> [(&str, &str); 7] {
        [(self.quit.as_str(), "quit")
        ,(self.play.as_str(), "play")
        ,(self.add_and_play.as_str(), "add and play")
        ,(self.up.as_str(), "up")
        ,(self.down.as_str(), "down")
        ,(self.add_to_queue.as_str(), "add to queue")
        ,(self.remove_from_queue.as_str(), "remove from queue")]
    }
}

impl AudioKeybindings {
    pub fn get_keybindings(&self) -> [(&str, &str); 8] {
        [(self.quit.as_str(), "quit")
        ,(self.pause.as_str(), "pause")
        ,(self.mute.as_str(), "mute")
        ,(self.volume_up.as_str(), "volume up")
        ,(self.volume_down.as_str(), "volume down")
        ,(self.speed_up.as_str(), "speed up")
        ,(self.speed_down.as_str(), "speed down")
        ,(self.reset_speed.as_str(), "reset speed")
        ]
    }
}
