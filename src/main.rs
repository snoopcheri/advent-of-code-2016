#[cfg(test)]
#[macro_use]
extern crate hamcrest;

use std::io::prelude::*;
use std::fs::File;

mod puzzle_example;
use puzzle_example::puzzle_example::PuzzleExample;

mod puzzle_day_1;
use puzzle_day_1::puzzle_day_1::PuzzleDay1;

mod puzzle_day_2;

use puzzle_day_2::puzzle_day_2::PuzzleDay2;


fn main() {
    puzzle_example();
    puzzle_day_1();
    puzzle_day_2();
}


fn puzzle_example() {
    let floor = PuzzleExample::new().solve_for(read_file("src/puzzle_example/input.txt").as_str());

    println!("puzzle_example: floor={}", floor);
}

fn puzzle_day_1() {
    let position = PuzzleDay1::new().solve_for(read_file("src/puzzle_day_1/input.txt").as_str());

    println!("puzzle_day_1: distance to origin={}", position.point().distance_from_origin());
    println!("puzzle_day_1: distance to origin of first already visited point={}", position.first_already_visited_point().unwrap().distance_from_origin());
}

fn puzzle_day_2() {
    let (regular_code, silly_code) = PuzzleDay2::new().solve_for(read_file("src/puzzle_day_2/input.txt").as_str());

    println!("puzzle_day_2: regular code={}", regular_code);
    println!("puzzle_day_2: silly code={}", silly_code);
}

fn read_file(filename: &str) -> String {
    let mut input_file: File= File::open(filename).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    return input_string;
}
