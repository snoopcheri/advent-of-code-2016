use super::position::*;

pub struct PuzzleDay1 {}


impl PuzzleDay1 {
    pub fn new() -> PuzzleDay1 {
        PuzzleDay1 {}
    }

    pub fn solve_for(&self, input: &str) -> Position {
        let mut position = Position::new();
        let mut steps = 0;

        for (idx, ch) in input.chars().enumerate() {
            match ch {
                'L' => position.turn_left(),
                'R' => position.turn_right(),
                '0' ... '9' => steps = steps * 10 + (ch as u32) - ('0' as u32),
                ',' => {
                    position.advance(steps);
                    steps = 0;
                },
                ' ' => continue,
                _ => panic!("invalid character {} at position {}", ch, idx),
            }
        }

        if steps != 0 {
            position.advance(steps);
        }

        position
    }
}


#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use super::*;

    #[test]
    fn test() {
        let puzzle = PuzzleDay1::new();

        assert_that!(puzzle.solve_for("R2, L3").point().distance_from_origin(), is(equal_to(5)));
        assert_that!(puzzle.solve_for("R2, R2, R2").point().distance_from_origin(), is(equal_to(2)));
        assert_that!(puzzle.solve_for("R5, L5, R5, R3").point().distance_from_origin(), is(equal_to(12)));
    }

    #[test]
    fn test_first_already_visited_point() {
        let puzzle = PuzzleDay1::new();

        let position = puzzle.solve_for("R8, R4, R4, R8");

        assert_that!(position.first_already_visited_point().unwrap().distance_from_origin(), is(equal_to(4)));
    }
}