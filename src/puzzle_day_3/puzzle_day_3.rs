use itertools::Itertools;
use super::triangle::*;
use super::parser::sides_of_triangle;


pub struct PuzzleDay3 {}

impl PuzzleDay3 {
    pub fn new() -> PuzzleDay3 {
        PuzzleDay3 {}
    }

    pub fn solve_for(&self, input: &str) -> usize {
        input.lines()
            .map(|line| sides_of_triangle(line))
            .map(|(a, b, c)| Triangle::new(a, b, c))
            .filter(|triangle| triangle.is_valid())
            .count()
    }

    pub fn solve_vertically_for(&self, input: &str) -> usize {
        input.lines()
            .tuples::<(_, _, _)>()
            .map(|(line1, line2, line3)| (sides_of_triangle(line1), sides_of_triangle(line2), sides_of_triangle(line3)))
            .flat_map(|((a, b, c), (d, e, f), (g, h, i))| vec!(Triangle::new(a, d, g), Triangle::new(b, e, h), Triangle::new(c, f, i)))
            .filter(|triangle| triangle.is_valid())
            .count()
    }
}


#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use super::*;

    #[test]
    fn test() {
        let puzzle = PuzzleDay3::new();

        assert_that!(puzzle.solve_for(" 5  10   25"), is(equal_to(0)));
    }

    #[test]
    fn test_vertical() {
        let input = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";

        let puzzle = PuzzleDay3::new();
        assert_that!(puzzle.solve_vertically_for(input), is(equal_to(6)));
    }
}
