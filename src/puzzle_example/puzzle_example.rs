pub struct PuzzleExample {}


impl PuzzleExample {
    pub fn new() -> PuzzleExample {
        PuzzleExample {}
    }

    pub fn solve_for(&self, input: &str) -> i32 {
        let mut number_of_open_brackets = 0;
        let mut number_of_closed_brackets = 0;

        for (idx, ch) in input.chars().enumerate() {
            match ch {
                '(' => number_of_open_brackets = number_of_open_brackets + 1,
                ')' => number_of_closed_brackets = number_of_closed_brackets + 1,
                _ => panic!("invalid character {} at position {}", ch, idx)
            }
        }

        number_of_open_brackets - number_of_closed_brackets
    }
}


#[cfg(test)]
mod tests {
    use super::super::super::hamcrest::prelude::*;
    use super::*;

    #[test]
    fn test() {
        let puzzle = PuzzleExample::new();

        assert_that!(puzzle.solve_for("(())"), is(equal_to(0)));
        assert_that!(puzzle.solve_for("()()"), is(equal_to(0)));

        assert_that!(puzzle.solve_for("((("), is(equal_to(3)));
        assert_that!(puzzle.solve_for("(()(()("), is(equal_to(3)));

        assert_that!(puzzle.solve_for("())"), is(equal_to(-1)));
        assert_that!(puzzle.solve_for("))("), is(equal_to(-1)));

        assert_that!(puzzle.solve_for(")))"), is(equal_to(-3)));
        assert_that!(puzzle.solve_for(")())())"), is(equal_to(-3)));
    }
}