use std::collections::HashSet;


#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}


impl Point {
    pub fn distance_from_origin(&self) -> u32 {
        (self.x.abs() as u32) + (self.y.abs() as u32)
    }
}


#[derive(Debug)]
pub struct Position {
    point: Point,
    direction: Direction,
    visited_points: Vec<Point>,
}


impl Position {
    pub fn new() -> Position {
        Position {
            point: Point { x: 0, y: 0 },
            direction: Direction::North,
            visited_points: Vec::new(),
        }
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        };
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::West => self.direction = Direction::South,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
        };
    }

    pub fn advance(&mut self, steps: u32) {
        for _i in 0..steps {
            self.advance_single_step();
        }
    }

    fn advance_single_step(&mut self) {
        match self.direction {
            Direction::North => self.point.y += 1,
            Direction::East => self.point.x += 1,
            Direction::South => self.point.y -= 1,
            Direction::West => self.point.x -= 1,
        };

        self.visited_points.push(self.point);
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn first_already_visited_point(&self) -> Option<Point> {
        let mut points: HashSet<Point> = HashSet::new();

        for point in self.visited_points.iter() {
            if points.contains(&point) {
                return Some(*point);
            }
            points.insert(*point);
        }

        None
    }
}
