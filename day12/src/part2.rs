use std::collections::{HashMap, HashSet};
use crate::part1::{Coordinate, Region};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Side {
    Up,
    Down,
    Left,
    Right,
}

impl Coordinate {
    fn neighbour_on_side(&self, side: Side) -> Self {
        match side {
            Side::Up => {Coordinate::new(self.horizontal, self.vertical-1)}
            Side::Down => {Coordinate::new(self.horizontal, self.vertical+1)}
            Side::Left => {Coordinate::new(self.horizontal-1, self.vertical)}
            Side::Right => {Coordinate::new(self.horizontal+1, self.vertical)}
        }
    }
}

impl Region {
    fn sides(&self) -> u32 {
        let mut side_set: HashSet<(Coordinate, Side)> = HashSet::new();

        for plot in &self.plots {
            for s in [Side::Up, Side::Down, Side::Left, Side::Right] {
                if !self.plots.contains(&plot.neighbour_on_side(s)) {
                    side_set.insert((*plot, s));
                }
            }
        }

        let mut grouped_sides: HashMap<(Coordinate,Side), i32> = HashMap::new();
        for side_entry in side_set.iter() {
            let mut current = side_entry.0;

            match side_entry.1 {
                Side::Up|Side::Down => {
                    let mut to_the_left = current.neighbour_on_side(Side::Left);
                    // find leftmost
                    while side_set.contains(&(to_the_left, side_entry.1)) {
                        current = to_the_left;
                        to_the_left = current.neighbour_on_side(Side::Left);
                    }

                    // current is the leftmost coordinate for the side. side_entry.1 is either
                    // Up or Down.
                    let key = (current, side_entry.1);

                    // if we have found one of the other wall pieces of the same wall, we should get
                    // to the same key because everything is normalised to the left and to bottom
                    if grouped_sides.contains_key(&key) {
                        continue;
                    }

                    let mut side_length = 1;
                    let mut to_the_right = current.neighbour_on_side(Side::Right);

                    // once we moved all the way to the left, check that if the space to the right
                    // also has the same border.
                    while side_set.contains(&(to_the_right, side_entry.1)) {
                        current = to_the_right;
                        to_the_right = current.neighbour_on_side(Side::Right);
                        side_length += 1;
                    }

                    grouped_sides.insert(key, side_length);
                }
                Side::Left|Side::Right => {
                    // find the bottom most
                    let mut below = current.neighbour_on_side(Side::Down);
                    while side_set.contains(&(below, side_entry.1)) {
                        current = below;
                        below = current.neighbour_on_side(Side::Down);
                    }

                    // current is the leftmost coordinate for the side. side_entry.1 is either
                    // Left or Right.
                    let key = (current, side_entry.1);

                    // if we have found one of the other wall pieces of the same wall, we should get
                    // to the same key because everything is normalised to the left and to bottom
                    if grouped_sides.contains_key(&key) {
                        continue;
                    }

                    let mut side_length = 1;
                    let mut above = current.neighbour_on_side(Side::Up);
                    while side_set.contains(&(above, side_entry.1)) {
                        current = above;
                        above = current.neighbour_on_side(Side::Up);
                        side_length += 1;
                    }

                    grouped_sides.insert(key, side_length);
                }
            }
        }

        grouped_sides.len() as u32
    }
}

pub(crate) fn solve(input: &str) -> u32 {
    let lines = input.trim().lines();
    let mut map: HashMap<Coordinate, String> = HashMap::new();

    for (height, line) in lines.enumerate() {
        for (width, c) in line.chars().enumerate() {
            map.insert(Coordinate::new(width as i32, height as i32), c.to_string());
        }
    }

    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut regions : Vec<Region> = Vec::new();

    for coord in map.keys() {
        let region = crate::part1::flood_fill(&map, *coord, map.get(coord).unwrap(), &mut visited);

        regions.push(Region::new(region));
    }

    let regions: Vec<&Region> = regions.iter().filter(|&r| !r.plots.is_empty()).collect();

    let mut sum = 0;

    for region in regions {
        sum += region.sides() * region.area() as u32
    }
    //
    // if let Some(&ref region) = regions.into_iter().next() {
    //     if let Some(plot) = region.plots.clone().into_iter().next() {
    //         println!("looking at region for {:?}:\ncoords:\n{:?}", plot, region.plots);
    //
    //     }
    //     let sides = region.sides();
    //
    //     println!("these are the sides: {:?}", sides);
    // }
    // //c
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_sides() {
        let input = "XX\nXY\n";

    }
}