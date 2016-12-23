use super::num_pad::*;
use super::regular_num_pad::*;
use super::silly_num_pad::*;


pub struct PuzzleDay2 {}


impl PuzzleDay2 {
    pub fn new() -> PuzzleDay2 {
        PuzzleDay2 {}
    }

    pub fn solve_for(&self, input: &str) -> (String, String) {
        let regular_code = replay_input_on_num_pad(input, &mut RegularNumPad::new());
        let silly_code = replay_input_on_num_pad(input, &mut SillyNumPad::new());

        (regular_code, silly_code)
    }
}


fn replay_input_on_num_pad(input: &str, num_pad: &mut NumPad) -> String {
    let mut num_pad_code = String::new();

    for (idx, ch) in input.chars().enumerate() {
        match ch {
            'L' => num_pad.move_left(),
            'R' => num_pad.move_right(),
            'U' => num_pad.move_up(),
            'D' => num_pad.move_down(),
            '\n' => {
                num_pad_code.push(num_pad.current_digit());
            }
            _ => panic!("invalid character {} at position {}", ch, idx),
        }
    }

    num_pad_code
}


#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use super::*;

    #[test]
    fn test() {
        let puzzle = PuzzleDay2::new();
        let (regular_code, silly_code) = puzzle.solve_for("ULL\nRRDDD\nLURDL\nUUUUD\n");

        assert_that!(regular_code.as_str(), is(equal_to("1985")));
        assert_that!(silly_code.as_str(), is(equal_to("5DB3")));
    }
}
