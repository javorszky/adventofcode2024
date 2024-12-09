use std::collections::{HashMap, HashSet};
use crate::Coordinate;

struct Day08 {
    last_h_idx: usize,
    last_w_idx: usize,
    antennae: HashMap<char, Vec<Coordinate>>,
    antinodes: HashMap<char, Vec<Coordinate>>
}

impl Day08 {
    fn new(input: &str) -> Day08 {
        let field: char = ".".parse().unwrap();

        let mut antennae = HashMap::new();
        let mut last_h_idx:usize = 0;
        let mut last_w_idx = 0;

        for (height, line) in input.trim().lines().enumerate() {
            last_h_idx = height;
            last_w_idx = line.len()-1; // len is not zero indexed

            for (width, ch) in line.trim().chars().enumerate() {
                let c = Coordinate{ height: height as i32, width: width as i32 };
                if !ch.eq(&field) {antennae.entry(ch).or_insert(Vec::new()).push(c);}
            }
        }

        Day08 {
            last_h_idx,
            last_w_idx,
            antennae,
            antinodes: HashMap::new()
        }
    }

    fn find_antinodes_per_antenna(&mut self) {
        for (ch, coords) in self.antennae.iter() {
            for pair in crate::part1::generate_all_node_pairs(coords.to_owned()) {
                let nodes = generate_antinodes(pair, self.last_h_idx, self.last_w_idx);

                nodes.iter().for_each(|node| self.antinodes.entry(*ch).or_default().push(node.to_owned()));
            }
        }
    }

    fn count_unique_antennae(&self) -> usize {
        let mut unique_antennae: HashSet<Coordinate> = HashSet::new();

        for coords in self.antinodes.values() {
            for coord in coords {
                unique_antennae.insert(*coord);
            }
        }

        unique_antennae.len()
    }
}


/// This function will generate antinodes that fall within the 0-height (inclusive) and
/// 0-width (inclusive) coordinate ranges.
fn generate_antinodes(coord_pair: (Coordinate, Coordinate), max_h: usize, max_w: usize) -> Vec<Coordinate> {
    let diff_height = coord_pair.0.height - coord_pair.1.height;
    let diff_width = coord_pair.0.width-coord_pair.1.width;

    // let's start by adding the two incoming into the vec, because they're absolutely nodes.
    let mut nodes = vec![coord_pair.0, coord_pair.1];

    let mut reference_node = coord_pair.0;

    loop {
        // first let's loop from first coordinate +
        let new_c1 = Coordinate {
            height: reference_node.height + diff_height,
            width: reference_node.width + diff_width
        };

        if !is_coord_within_map(new_c1, max_h, max_w) {
            break;
        }

        nodes.push(new_c1);
        reference_node = new_c1;
    }

    reference_node = coord_pair.1;

    loop {
        // first let's loop from first coordinate +
        let new_c1 = Coordinate {
            height: reference_node.height - diff_height,
            width: reference_node.width - diff_width
        };

        if !is_coord_within_map(new_c1, max_h, max_w) {
            break;
        }

        nodes.push(new_c1);
        reference_node = new_c1;
    }

    nodes
}

fn is_coord_within_map(coordinate: Coordinate, max_h: usize, max_w: usize) -> bool {
    if coordinate.width >= 0
        && coordinate.height >= 0
        && coordinate.width as usize <= max_w
        && coordinate.height as usize <= max_h
    {
        return true;
    }

    false
}

pub(crate) fn solve(input: &str) -> usize {
    let mut d = Day08::new(input);

    d.find_antinodes_per_antenna();

    d.count_unique_antennae()
}