use std::collections::HashMap;
use crate::{next_coord, parse_char_to_tile, Coordinate, Direction, Tile, OutOfMapError};

struct Day06 {
    map: HashMap<Coordinate, Tile>,
    visited: HashMap<Coordinate, Tile>,
    obstacles: HashMap<Coordinate, Tile>,
    current_coordinate: Coordinate,
    starting_coordinate: Coordinate,
    direction: Direction,
    logs: Vec<String>,
}


impl Day06 {
    fn new(data: &str) -> Day06 {
        let mut map: HashMap<Coordinate, Tile> = HashMap::new();
        let mut obstacles: HashMap<Coordinate, Tile> = HashMap::new();

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
                    }
                    Tile::GuardLeft => {
                        starting_coordinate = coord;
                        starting_direction = Direction::Left;
                    }
                    Tile::GuardRight => {
                        starting_coordinate = coord;
                        starting_direction = Direction::Right;
                    }
                    Tile::GuardUp => {
                        starting_coordinate = coord;
                        starting_direction = Direction::Up;
                    }
                    Tile::Obstacle => {
                        obstacles.insert(coord, tile);
                    }
                    _ => {}
                }

                // println!("inserting {} at {:?}", tile, coord);

                map.insert(coord, tile);
            }
        }

        Day06 {
            map,
            obstacles,
            current_coordinate: starting_coordinate,
            starting_coordinate,
            direction: starting_direction,
            visited: HashMap::from([(starting_coordinate, Tile::Visited)]),
            logs: vec![format!(
                "Starting at coordinate {:?} with guard facing {:?}",
                starting_coordinate, starting_direction
            )],
        }
    }

    fn next(&mut self) -> Result<(), OutOfMapError> {
        let new_coordinate: Coordinate = next_coord(&self.current_coordinate, &self.direction);

        let new_tile = self.map.get(&new_coordinate).unwrap_or(&Tile::Outside);

        match *new_tile {
            Tile::Outside => {
                // println!("found an outside, breaking");
                return Err(OutOfMapError);
            }
            Tile::Obstacle => {
                // rotate 90deg to the right, stay in place
                self.direction = self.direction.turn90();
                // also record the new turned visited
                self.logs
                    .push(format!("Obstacle at {:?}, turning right", new_coordinate))
            }
            _ => {
                // move to that tile, do not change orientation, mark it as visited
                self.current_coordinate = new_coordinate;
                self.visited.insert(new_coordinate, Tile::Visited);
                self.logs.push(format!(
                    "Floor / Visited / Guard at {:?}, going forward, marking visited",
                    new_coordinate
                ))
            }
        }

        Ok(())
    }

    fn walk(&mut self) {
        loop {
            if self.next().is_err() {
                break;
            }
        }
    }

    fn what_happened(&self) -> String {
        self.logs.join("\n")
    }

    fn visited(&self) -> i32 {
        self.visited.len() as i32
    }
}


pub fn part1(data: &str) -> i32 {
    let mut day = Day06::new(data);
    day.walk();

    day.visited()
}
