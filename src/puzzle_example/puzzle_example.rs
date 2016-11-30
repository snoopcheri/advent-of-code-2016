pub struct PuzzleExample {}


impl PuzzleExample {
    pub fn new() -> PuzzleExample {
        PuzzleExample {}
    }

    pub fn solve_for(&self, input: &str) -> i32 {
        let mut floor = 0;

        for (idx, ch) in input.chars().enumerate() {
            match ch {
                '(' => floor = floor + 1,
                ')' => floor = floor - 1,
                _ => panic!("invalid character {} at position {}", ch, idx)
            }
        }

        floor
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