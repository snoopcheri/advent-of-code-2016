extern crate regex;

#[macro_use] extern crate itertools;

#[cfg(test)]
#[macro_use] extern crate hamcrest;

#[macro_use] extern crate nom;

use std::io::prelude::*;
use std::fs::File;

mod puzzle_example;

use puzzle_example::*;

mod puzzle_day_1;

use puzzle_day_1::*;

mod puzzle_day_2;

use puzzle_day_2::*;

mod puzzle_day_3;

use puzzle_day_3::*;

mod puzzle_day_4;

use puzzle_day_4::*;


fn main() {
    puzzle_example();
    puzzle_day_1();
    puzzle_day_2();
    puzzle_day_3();
    puzzle_day_4();
}


fn puzzle_example() {
    let floor = PuzzleExample::new().solve_for(read_file("src/puzzle_example/input.txt").as_str());

    println!("puzzle_example: floor={}", floor);
}

fn puzzle_day_1() {
    let position = PuzzleDay1::new().solve_for(read_file("src/puzzle_day_1/input.txt").as_str());

    println!("puzzle_day_1: distance to origin={}",
             position.point().distance_from_origin());
    println!("puzzle_day_1: distance to origin of first already visited point={}",
             position.first_already_visited_point().unwrap().distance_from_origin());
}

fn puzzle_day_2() {
    let (regular_code, silly_code) = PuzzleDay2::new()
        .solve_for(read_file("src/puzzle_day_2/input.txt").as_str());

    println!("puzzle_day_2: regular code={}", regular_code);
    println!("puzzle_day_2: silly code={}", silly_code);
}

fn puzzle_day_3() {
    let valid_triangles = PuzzleDay3::new()
        .solve_for(read_file("src/puzzle_day_3/input.txt").as_str());
    let valid_vertical_triangles = PuzzleDay3::new()
        .solve_vertically_for(read_file("src/puzzle_day_3/input.txt").as_str());

    println!("puzzle_day_3: #(valid triangles)={}", valid_triangles);
    println!("puzzle_day_3: #(valid vertical triangles)={}",
             valid_vertical_triangles);
}

fn puzzle_day_4() {
    let answer = PuzzleDay4::new().solve_for(read_file("src/puzzle_day_4/input.txt").as_str());

    println!("puzzle_day_4: answer={}", answer);
}

fn read_file(filename: &str) -> String {
    let mut input_file: File = File::open(filename).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    return input_string;
}
