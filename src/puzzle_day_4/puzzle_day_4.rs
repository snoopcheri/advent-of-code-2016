use regex::Regex;
use super::room::*;

pub struct PuzzleDay4 {}

impl PuzzleDay4 {
    pub fn new() -> PuzzleDay4 {
        PuzzleDay4 {}
    }

    pub fn solve_for(&self, input: &str) -> u64 {
        let room_regex: Regex = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();

        input.lines()
            .map(|line| to_room(line, &room_regex))
            .filter(|room| room.is_checksum_valid())
            .map(|room| room.sector_id())
            .fold(0, |total, next| total + next)
    }
}


fn to_room(line: &str, room_regex: &Regex) -> Room {
    let caps = room_regex.captures_iter(line).next().unwrap();

    Room::new(
        caps.at(1).unwrap().parse::<String>().unwrap().as_str(),
        caps.at(2).unwrap().parse::<u64>().unwrap(),
        caps.at(3).unwrap().parse::<String>().unwrap().as_str(),
    )
}


#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use super::*;

    #[test]
    fn test() {
        let input = "aaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

        let puzzle = PuzzleDay4::new();

        assert_that!(puzzle.solve_for(input), is(equal_to(1514)));
    }
}
