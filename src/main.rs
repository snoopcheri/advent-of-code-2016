#[cfg(test)]
#[macro_use]
extern crate hamcrest;

use std::io::prelude::*;
use std::fs::File;

mod puzzle_example;
use puzzle_example::puzzle_example::PuzzleExample;


fn main() {
    let puzzle = PuzzleExample::new();
    let floor = puzzle.solve_for(read_file("src/puzzle_example/input.txt").as_str());

    println!("floor={}", floor);
}

fn read_file(filename: &str) -> String {
    let mut input_file: File= File::open(filename).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    return input_string;
}
