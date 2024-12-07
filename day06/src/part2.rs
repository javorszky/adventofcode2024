use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use crate::{next_coord, parse_char_to_tile, Coordinate, Direction, Tile, OutOfMapError};

struct LoopError;

impl Display for LoopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Guard entered a loop")
    }
}

enum WalkError {
    OutOfMapError,
    LoopError
}


#[derive(Debug)]
struct Candidate {
    place_obstacle_at: Coordinate,
    while_going: Direction,
    at_coordinate: Coordinate
}

struct P2Day06 {
    map: HashMap<Coordinate, Tile>,
    visited: HashSet<Coordinate>,
    visited_with_direction: HashMap<(Coordinate, Direction), Tile>,
    visited_with_direction_vec: Vec<(Coordinate, Direction)>, // so we have this in order
    current_coordinate: Coordinate,
    starting_coordinate: Coordinate,
    direction: Direction,
    logs: Vec<String>,
}

impl P2Day06 {
    fn new(data: &str) -> P2Day06 {
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
                    _ => {}
                }

                map.insert(coord, tile);
            }
        }

        P2Day06 {
            map,
            current_coordinate: starting_coordinate,
            starting_coordinate,
            direction: starting_direction,
            visited: HashSet::new(),
            visited_with_direction: HashMap::from([(
                (starting_coordinate, starting_direction),
                Tile::Visited,
            )]),
            visited_with_direction_vec: vec![(starting_coordinate, starting_direction)],
            logs: vec![format!(
                "Starting at coordinate {:?} with guard facing {:?}",
                starting_coordinate, starting_direction
            )],
        }
    }

    fn next(&mut self) -> Result<(), OutOfMapError> {
        let new_coordinate: Coordinate = next_coord(&self.current_coordinate, &self.direction);

        // Record where we've just been
        self.visited_with_direction.insert((self.current_coordinate, self.direction), Tile::Visited);
        self.visited_with_direction_vec.push((self.current_coordinate, self.direction));
        self.visited.insert(self.current_coordinate);

        let new_tile = self.map.get(&new_coordinate).unwrap_or(&Tile::Outside);
        match *new_tile {
            Tile::Outside => {
                return Err(OutOfMapError);
            }
            Tile::Obstacle => {
                // rotate 90deg to the right, stay in place
                self.direction = self.direction.turn90();

                self.logs
                    .push(format!("Obstacle at {:?}, turning right", new_coordinate))
            }
            _ => {
                // move to that tile, do not change orientation
                self.current_coordinate = new_coordinate;
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

    fn next_check_loop(&mut self) -> Result<(), WalkError> {
        let nc = next_coord(&self.current_coordinate, &self.direction);
        let new_tile = self.map.get(&nc).unwrap_or(&Tile::Outside);

        self.visited_with_direction.insert((self.current_coordinate, self.direction), Tile::Visited);
        self.visited_with_direction_vec.push((self.current_coordinate, self.direction));

        match *new_tile {
            Tile::Outside => {
                return Err(WalkError::OutOfMapError)
            }
            Tile::Obstacle => {
                // rotate 90deg to the right, stay in place
                self.direction = self.direction.turn90();

                self.logs
                    .push(format!("Obstacle at {:?}, turning right", nc))
            }
            _ => {
                // first let's see whether we've been where we want to move
                if self.visited_with_direction.contains_key(&(nc, self.direction)) {
                    self.logs.push(format!(
                        "We have already visited the tile at {:?} going {:?}", nc, self.direction)
                    );
                    // this is what we want
                    return Err(WalkError::LoopError)
                }

                // move to that tile, do not change orientation, mark it as visited
                self.current_coordinate = nc;
                self.logs.push(format!(
                    "Floor / Visited / Guard at {:?}, going forward, marking visited",
                    nc
                ))
            }
        }

        Ok(())
    }

    fn walk_loop(&mut self) -> Result<(), LoopError> {
        loop {
            if let Err(e) = self.next_check_loop() {
                return match e {
                    WalkError::LoopError => {
                        Err(LoopError)
                    },
                    WalkError::OutOfMapError => {
                        Ok(())
                    }
                }
            }
        }
    }

    fn what_happened(&self) -> String {
        self.logs.join("\n")
    }

    fn does_it_hit_obstacle(&self, coord: Coordinate, dir: Direction) ->  bool {
        let mut new_coord = coord;

        loop {
            new_coord = next_coord(&new_coord, &dir);
            let nt = self.map.get(&new_coord).unwrap_or(&Tile::Outside);

            match nt {
                Tile::Outside => {
                    return false
                },
                Tile::Obstacle => {
                    return true
                }
                _ => {
                    // do nothing, this should always break because going the same direction will
                    // always either hit an obstacle, or at some point we're going to go off the map
                }
            }
        }
    }
}

fn should_skip_next_tile(day: &P2Day06, coordinate: &Coordinate) -> bool {
    let next_tile = day.map.get(coordinate).unwrap_or(&Tile::Outside);
    if *next_tile == Tile::Outside
        || *next_tile == Tile::Obstacle
        || *next_tile == Tile::GuardUp {
        // if the next tile is an obstacle already, or outside, or a guard, then I won't
        // be able to place a paradox object here.
        return true;
    }

    false
}


pub fn part2(data: &str) -> i32 {
    let mut day = P2Day06::new(data);
    // walk the first time, as usual
    day.walk();

    let mut candidates : Vec<Candidate> = Vec::new();
    let mut checked: HashSet<Coordinate> = HashSet::new();

    for (c, d) in day.visited_with_direction_vec.iter() {
        // store current coordinate in a new "visited" vec that we're growing. This is used to check
        // whether we'd try to put an obstacle onto a path we have been on which would mean the
        // guard would not reach the floor she's on at the moment because her path would have been
        // broken earlier.
        checked.insert(*c);

        let nc = next_coord(c, d);
        if should_skip_next_tile(&day, &nc) {
            continue;
        }

        if checked.contains(&nc) {
            continue
        }

        if day.does_it_hit_obstacle(*c, d.turn90()) {
            let cand = Candidate{
                place_obstacle_at: nc, // new obstacle is here
                while_going: *d, // starting direction is here
                at_coordinate: *c, // starting coordinate is here
            };
            candidates.push(cand);
        }
    }

    let mut unique_placements:HashSet<Coordinate> = HashSet::new();

    for candidate in candidates {
        let mut m = day.map.clone();

        match m.insert(candidate.place_obstacle_at, Tile::Obstacle) {
            None => print!("replacing floor to obstacle at {:?} failed", candidate.place_obstacle_at),
            Some(old) => {
                if old != Tile::Floor {
                    println!("something very wrong happened, we're updating the tile to an obstacle, but the old one is a {:?}", old)
                }
            }
        }

        // create a copy candidate of the original.
        let mut candidate_day = P2Day06 {
            map: m,
            visited_with_direction: HashMap::new(),
            visited_with_direction_vec: Vec::new(),
            visited: HashSet::new(),
            current_coordinate: candidate.at_coordinate,
            starting_coordinate: candidate.at_coordinate,
            direction: candidate.while_going,
            logs: vec![],
        };

        if candidate_day.walk_loop().is_err() {
            unique_placements.insert(candidate.place_obstacle_at);
        }
    }

    unique_placements.remove(&(day.starting_coordinate.0, day.starting_coordinate.1));

    unique_placements.len() as i32
}

fn direction_to_guard(dir: &Direction) -> Tile {
    match dir {
        Direction::Up => {Tile::GuardUp}
        Direction::Left => {Tile::GuardLeft}
        Direction::Down => {Tile::GuardDown}
        Direction::Right => {Tile::GuardRight}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day6_loops() {
        let data: &str = concat!(
        ".#..\n",
        ".^.#\n",
        "#...\n",
        "..#.\n",
        );

        let mut d6 = P2Day06::new(data);
        let res = d6.walk_loop();

        assert!(res.is_err());
    }
}
