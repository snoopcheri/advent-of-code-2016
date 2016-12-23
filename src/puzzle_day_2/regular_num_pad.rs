use std::cmp::{min, max};

use super::num_pad::NumPad;


pub struct RegularNumPad {
    x: i8,
    y: i8,
}


impl RegularNumPad {
    pub fn new() -> RegularNumPad {
        RegularNumPad { x: 1, y: 1 }
    }
}

impl NumPad for RegularNumPad {
    fn move_right(&mut self) {
        self.x = min(self.x + 1, 2);
    }

    fn move_left(&mut self) {
        self.x = max(self.x - 1, 0);
    }

    fn move_up(&mut self) {
        self.y = max(self.y - 1, 0);
    }

    fn move_down(&mut self) {
        self.y = min(self.y + 1, 2);
    }

    fn current_digit(&self) -> char {
        ((self.x + 1) + (self.y * 3) + ('0' as i8)) as u8 as char
    }
}
