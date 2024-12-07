#![allow(dead_code)]

mod part2;
mod part1;

use std::cmp::{PartialEq};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

type Coordinate = (i32, i32);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Floor,
    Obstacle,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
    Visited,
    Outside,
    Paradox
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Floor => write!(f, "."),
            Tile::Obstacle => write!(f, "#"),
            Tile::GuardUp => write!(f, "^"),
            Tile::GuardDown => write!(f, "v"),
            Tile::GuardLeft => write!(f, "<"),
            Tile::GuardRight => write!(f, ">"),
            Tile::Visited => write!(f, "X"),
            Tile::Outside => write!(f, "~"),
            Tile::Paradox => write!(f, "O"),
        }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn turn90(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct OutOfMapError;

impl Display for OutOfMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Guard left the map")
    }
}

fn next_coord(coordinate: &Coordinate, dir: &Direction) -> Coordinate {
    match dir {
        Direction::Up => {
            (coordinate.0-1, coordinate.1)
        }
        Direction::Left => {
            (coordinate.0, coordinate.1-1)
        }
        Direction::Down => {
            (coordinate.0+1, coordinate.1)
        }
        Direction::Right => {
            (coordinate.0, coordinate.1+1)
        }
    }
}

fn parse_char_to_tile(c: char) -> Tile {
    match c.to_string().as_str() {
        "#" => Tile::Obstacle,
        "." => Tile::Floor,
        "^" => Tile::GuardUp,
        ">" => Tile::GuardRight,
        "<" => Tile::GuardLeft,
        "v" => Tile::GuardDown,
        "X" => Tile::Visited,
        _ => {
            panic!("Unknown character: {}", c);
        }
    }
}

pub fn solve_part1_example() -> i32 {
    let data: &str = concat!(
        "....#.....\n",
        ".........#\n",
        "..........\n",
        "..#.......\n",
        ".......#..\n",
        "..........\n",
        ".#..^.....\n",
        "........#.\n",
        "#.........\n",
        "......#...\n"
    );

    part1::part1(data)
}

pub fn solve_part1() -> i32 {
    part1::part1(include_str!("../input.txt"))
}

pub fn solve_part2_example() -> i32 {
    let data: &str = concat!(
        "....#.....\n",
        ".........#\n",
        "..........\n",
        "..#.......\n",
        ".......#..\n",
        "..........\n",
        ".#..^.....\n",
        "........#.\n",
        "#.........\n",
        "......#...\n"
    );

    part2::part2(data)
}

pub fn solve_part2() -> i32 {
    part2::part2(include_str!("../input.txt"))
}

