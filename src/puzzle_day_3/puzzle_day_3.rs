use regex::Regex;
use itertools::Itertools;
use super::triangle::*;


pub struct PuzzleDay3 {}

impl PuzzleDay3 {
    pub fn new() -> PuzzleDay3 {
        PuzzleDay3 {}
    }

    pub fn solve_for(&self, input: &str) -> usize {
        let numbers_regex: Regex = Regex::new(r"(\d+)[ ]+(\d+)[ ]+(\d+)").unwrap();

        input.split('\n')
            .map(|line| three_numbers(line, &numbers_regex))
            .map(|(a, b, c)| Triangle::new(a, b, c))
            .filter(|triangle| triangle.is_valid())
            .count()
    }

    pub fn solve_vertically_for(&self, input: &str) -> usize {
        let numbers_regex: Regex = Regex::new(r"(\d+)[ ]+(\d+)[ ]+(\d+)").unwrap();

        input.split('\n')
            .tuples::<(_, _, _)>()
            .map(|(line1, line2, line3)| (three_numbers(line1, &numbers_regex), three_numbers(line2, &numbers_regex), three_numbers(line3, &numbers_regex)))
            .flat_map(|((a, b, c), (d, e, f), (g, h, i))| vec!(Triangle::new(a, d, g), Triangle::new(b, e, h), Triangle::new(c, f, i)))
            .filter(|triangle| triangle.is_valid())
            .count()
    }
}


fn three_numbers(line: &str, numbers_regex: &Regex) -> (u32, u32, u32) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for caps in numbers_regex.captures_iter(line) {
        a = caps.at(1).unwrap().parse::<u32>().unwrap();
        b = caps.at(2).unwrap().parse::<u32>().unwrap();
        c = caps.at(3).unwrap().parse::<u32>().unwrap();
    }

        (a, b, c)
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
