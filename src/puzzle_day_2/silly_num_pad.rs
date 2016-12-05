use super::num_pad::NumPad;


pub struct SillyNumPad {
    x: i8,
    y: i8,
}


impl SillyNumPad {
    pub fn new() -> SillyNumPad {
        SillyNumPad { x: -2, y: 0 }
    }
}


impl NumPad for SillyNumPad {
    fn move_right(&mut self) {
        self.x = match is_in_distance(self.x + 1, self.y) {
            true => self.x + 1,
            false => self.x,
        }
    }

    fn move_left(&mut self) {
        self.x = match is_in_distance(self.x - 1, self.y) {
            true => self.x - 1,
            false => self.x,
        }
    }

    fn move_up(&mut self) {
        self.y = match is_in_distance(self.x, self.y - 1) {
            true => self.y - 1,
            false => self.y,
        }
    }

    fn move_down(&mut self) {
        self.y = match is_in_distance(self.x, self.y + 1) {
            true => self.y + 1,
            false => self.y,
        }
    }

    fn current_digit(&self) -> char {
        match (self.x, self.y) {
            (0, - 2) => '1',
            (- 1, - 1) => '2',
            (0, - 1) => '3',
            (1, - 1) => '4',
            (- 2, 0) => '5',
            (- 1, 0) => '6',
            (0, 0) => '7',
            (1, 0) => '8',
            (2, 0) => '9',
            (- 1, 1) => 'A',
            (0, 1) => 'B',
            (1, 1) => 'C',
            (0, 2) => 'D',
            _ => panic!("unexpected position (x={}, y={}) in silly num pad", self.x, self.y),
        }
    }
}


fn is_in_distance(x: i8, y: i8) -> bool {
    (x.abs() + y.abs()) <= 2
}
