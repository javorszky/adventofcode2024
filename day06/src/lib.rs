use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Coordinate is (height, width), 0 indexed, top left corner is (0, 0)
type Coordinate = (i32, i32);

struct OutOfMapError;

impl Display for OutOfMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Guard left the map")
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Floor,
    Obstacle,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
    Visited,
    Outside
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up, Left, Down, Right
}

impl Direction {
    fn turn90(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
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
            Tile::Outside => write!(f, "O")
        }
    }
}

struct Day06 {
    map: HashMap<Coordinate, Tile>,
    visited: HashMap<Coordinate, Tile>,
    current_coordinate: Coordinate,
    direction: Direction,
    logs: Vec<String>
}

impl Day06 {
    pub fn new(data: &str) -> Day06 {
        let mut map: HashMap<Coordinate, Tile> = HashMap::new();
        let mut starting_coordinate: Coordinate = (0, 0);
        let mut starting_direction: Direction = Direction::Up;

        for (height, line) in data.lines().enumerate() {
            for (width, character) in line.chars().enumerate() {
                let coord = (height as i32, width as i32);
                let tile = parse_char_to_tile(character);

                match tile {
                    Tile::GuardDown => {
                        starting_coordinate = coord;
                        starting_direction = Direction::Down;
                    },
                    Tile::GuardLeft => {
                        starting_coordinate = coord;
                        starting_direction = Direction::Left;
                    },
                    Tile::GuardRight => {
                        starting_coordinate = coord;
                        starting_direction = Direction::Right;
                    },
                    Tile::GuardUp => {
                        starting_coordinate = coord;
                        starting_direction = Direction::Up;
                    },
                    _ => {}
                }

                // println!("inserting {} at {:?}", tile, coord);

                map.insert(coord, tile);
            }
        }

        Day06 {
            map,
            current_coordinate: starting_coordinate,
            direction: starting_direction,
            visited: HashMap::from([
                (starting_coordinate, Tile::Visited)
            ]),
            logs: vec![
                format!("Starting at coordinate {:?} with guard facing {:?}", starting_coordinate, starting_direction )
            ]
        }
    }

    fn next(&mut self) -> Result<(), OutOfMapError> {
        let new_coordinate: Coordinate = match self.direction {
            Direction::Up => {
                (self.current_coordinate.0-1, self.current_coordinate.1)
            },
            Direction::Right => {
                (self.current_coordinate.0, self.current_coordinate.1+1)
            }
            Direction::Down => {
                (self.current_coordinate.0+1, self.current_coordinate.1)
            }
            Direction::Left => {
                (self.current_coordinate.0, self.current_coordinate.1-1)
            }
        };

        let new_tile = self.map.get(&new_coordinate).unwrap_or(&Tile::Outside);
        match *new_tile {
            Tile::Outside => {
                // println!("found an outside, breaking");
                return Err(OutOfMapError)
            },
            Tile::Obstacle => {
                // rotate 90deg to the right, stay in place
                self.direction = self.direction.turn90();
                self.logs.push(format!("Obstacle at {:?}, turning right", new_coordinate))
            },
            _ => {
                // move to that tile, do not change orientation, mark it as visited
                self.current_coordinate = new_coordinate;
                self.visited.insert(new_coordinate, Tile::Visited);
                self.logs.push(format!("Floor / Visited / Guard at {:?}, going forward, marking visited", new_coordinate))
            },
        }

        Ok(())
    }

    pub fn walk (&mut self) {
        loop {
            if self.next().is_err() {
                break
            }
        }
    }

    pub fn what_happened(&self) -> String {
        self.logs.join("\n")
    }

    pub fn visited(&self) -> i32 {
        self.visited.len() as i32
    }
}

pub fn solve_part1() -> i32 {
    part1(include_str!("../input.txt"))
}

fn part1(data: &str) -> i32 {
    let mut day = Day06::new(data);
    day.walk();

    day.visited()
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

    part1(data)
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