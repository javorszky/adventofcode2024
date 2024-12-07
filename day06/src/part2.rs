use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use crate::{next_coord, parse_char_to_tile, part2, Coordinate, Direction, Tile, OutOfMapError};

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
    visited_with_direction: HashMap<(Coordinate, Direction), Tile>,
    visited_with_direction_vec: Vec<(Coordinate, Direction)>, // so we have this in order
    obstacles_per_width_first: HashMap<i32, i32>,
    obstacles_per_height_first: HashMap<i32, i32>,
    current_coordinate: Coordinate,
    starting_coordinate: Coordinate,
    direction: Direction,
    logs: Vec<String>,
}

impl P2Day06 {
    fn new(data: &str) -> P2Day06 {
        let mut map: HashMap<Coordinate, Tile> = HashMap::new();
        let mut obstacles_per_width_first: HashMap<i32, i32> = HashMap::new();
        let mut obstacles_per_height_first: HashMap<i32, i32> = HashMap::new();

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
                        obstacles_per_height_first.insert(coord.0, coord.1);
                        obstacles_per_width_first.insert(coord.1, coord.0);
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
            visited_with_direction: HashMap::from([(
                (starting_coordinate, starting_direction),
                Tile::Visited,
            )]),
            visited_with_direction_vec: vec![(starting_coordinate, starting_direction)],
            obstacles_per_width_first,
            obstacles_per_height_first,
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

        let new_tile = self.map.get(&new_coordinate).unwrap_or(&Tile::Outside);
        match *new_tile {
            Tile::Outside => {
                // println!("found an outside, breaking");
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
                // println!("found an outside, breaking");
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
        // println!("checking for coord {:?} going {:?}", coord, dir);

        let mut new_coord = coord;

        loop {
            new_coord = next_coord(&new_coord, &dir);
            // println!("-- new coord is {:?}", new_coord);
            let nt = self.map.get(&new_coord).unwrap_or(&Tile::Outside);

            match nt {
                Tile::Outside => {
                    // println!("-- found outside, returning false");
                    return false
                },
                Tile::Obstacle => {
                    // println!("-- found obstacle, returning true");
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
        // println!("-- next tile ({:?}) is {:?}, onwards", coordinate, next_tile);
        return true;
    }

    false
}


pub fn part2(data: &str) -> i32 {
    let mut day = part2::P2Day06::new(data);
    // walk the first time, as usual
    day.walk();

    let mut candidates : Vec<Candidate> = Vec::new();

    for (c, d) in day.visited_with_direction_vec.iter() {
        // println!("\nOn coordinate {:?} going direction {:?}", c, d);

        let nc = next_coord(c, d);
        if should_skip_next_tile(&day, &nc) {
            // println!("-- skipped next tile found at {:?}", nc);
            continue;
        }

        if day.does_it_hit_obstacle(*c, d.turn90()) {
            let cand = Candidate{
                place_obstacle_at: nc,
                while_going: *d,
                at_coordinate: *c,
            };
            candidates.push(cand);
            // println!("-- adding candidate at {:?} dir {:?} starting {:?}", nc, d, c);
        }
    }

    let mut unique_placements:HashSet<Coordinate> = HashSet::new();

    for candidate in candidates {
        // println!("\nChecking for candidate at {:?}", candidate.place_obstacle_at);

        let mut m = day.map.clone();
        m.insert(candidate.place_obstacle_at, Tile::Obstacle);

        let mut obh = day.obstacles_per_height_first.clone();
        obh.insert(candidate.place_obstacle_at.0, candidate.place_obstacle_at.1);

        let mut obw = day.obstacles_per_width_first.clone();
        obw.insert(candidate.place_obstacle_at.1, candidate.place_obstacle_at.0);

        let mut candidate_day = P2Day06 {
            map: m,
            visited_with_direction: HashMap::new(),
            visited_with_direction_vec: Vec::new(),
            obstacles_per_width_first: obw,
            obstacles_per_height_first: obh,
            current_coordinate: candidate.at_coordinate,
            starting_coordinate: candidate.at_coordinate,
            direction: candidate.while_going,
            logs: vec![],
        };

        if candidate_day.walk_loop().is_err() {
            unique_placements.insert(candidate.place_obstacle_at);
            // println!("-- candidate caused a loop");
            // possibilities += 1;
        }
    }

    if unique_placements.contains(&(day.starting_coordinate.0, day.starting_coordinate.1)) {
        println!("unique placements has the starting coordinate in it...")
    }

    unique_placements.remove(&(day.starting_coordinate.0, day.starting_coordinate.1));

    if unique_placements.contains(&(day.starting_coordinate.0, day.starting_coordinate.1)) {
        println!("unique placements still has the starting coordinate in it even tho I removed it...")
    }


    // println!("paradox obstacles: {:?}", possibilities);

    unique_placements.len() as i32
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
