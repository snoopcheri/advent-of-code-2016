use super::num_pad::NumPad;


pub struct RegularNumPad {
    x: u8,
    y: u8,
}


impl RegularNumPad {
    pub fn new() -> RegularNumPad {
        RegularNumPad { x: 1, y: 1 }
    }
}

impl NumPad for RegularNumPad {
    fn move_right(&mut self) {
        self.x = match self.x {
            2 => self.x,
            _ => self.x + 1
        }
    }

    fn move_left(&mut self) {
        self.x = match self.x {
            0 => self.x,
            _ => self.x - 1,
        }
    }

    fn move_up(&mut self) {
        self.y = match self.y {
            0 => self.y,
            _ => self.y - 1,
        }
    }

    fn move_down(&mut self) {
        self.y = match self.y {
            2 => self.y,
            _ => self.y + 1,
        }
    }

    fn current_digit(&self) -> char {
        (((self.x + 1) + (self.y * 3)) + 48) as char
    }
}
