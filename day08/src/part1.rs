use std::collections::{HashMap, HashSet};
use crate::Coordinate;


struct Day08 {
    map: HashMap<Coordinate, char>,
    last_h_idx: usize,
    last_w_idx: usize,
    antennae: HashMap<char, Vec<Coordinate>>,
    antinodes: HashMap<char, Vec<Coordinate>>
}

impl Day08 {
    fn new(input: &str) -> Day08 {
        let field: char = ".".parse().unwrap();

        let mut map = HashMap::new();
        let mut antennae = HashMap::new();
        let mut last_h_idx:usize = 0;
        let mut last_w_idx = 0;

        for (height, line) in input.trim().lines().enumerate() {
            last_h_idx = height;
            last_w_idx = line.len()-1; // len is not zero indexed

            for (width, ch) in line.trim().chars().enumerate() {
                let c = Coordinate{ height: height as i32, width: width as i32 };
                map.insert(c, ch);
                if !ch.eq(&field) {antennae.entry(ch).or_insert(Vec::new()).push(c);}
            }
        }

        Day08 {
            map,
            last_h_idx,
            last_w_idx,
            antennae,
            antinodes: HashMap::new()
        }
    }

    fn find_antinodes_per_antenna(&mut self) {
        for (ch, coords) in self.antennae.iter() {
            for pair in generate_all_node_pairs(coords.to_owned()) {
                let nodes = generate_antinodes(pair);

                if self.is_coord_within_map(nodes.0) {
                    self.antinodes.entry(*ch).or_default().push(nodes.0);
                }

                if self.is_coord_within_map(nodes.1) {
                    self.antinodes.entry(*ch).or_default().push(nodes.1);
                }
            }
        }
    }

    fn count_unique_antennae(&self) -> usize {
        let mut unique_antennae: HashSet<Coordinate> = HashSet::new();

        for coords in self.antinodes.values() {
            for coord in coords {
                unique_antennae.insert(coord.clone());
            }
        }

        unique_antennae.len()
    }

    fn is_coord_within_map(&self, coordinate: Coordinate) -> bool {
        if coordinate.width >= 0
            && coordinate.height >= 0
            && coordinate.width as usize <= self.last_w_idx
            && coordinate.height as usize <= self.last_h_idx
        {
            return true;
        }

        false
    }


}

pub(crate) fn solve(input: &str) -> usize {
    let mut d = Day08::new(input);

    // println!("map:\n{:?}", d.map);
    // println!("\n\nantennae:\n{:?}", d.antennae);
    // println!("\nmax h: {}\n\
    // max w: {}", d.last_h_idx, d.last_w_idx);

    d.find_antinodes_per_antenna();

    // println!("\nantinodes:\n{:?}", d.antinodes);

    d.count_unique_antennae()
}

fn generate_all_node_pairs(coords: Vec<Coordinate>) -> Vec<(Coordinate, Coordinate)> {
    if coords.len() < 2 {
        return vec![];
    }

    let mut pairs = Vec::new();

    for (i, coord) in coords.iter().enumerate() {
        for other_coord in coords[i+1..].iter() {
            pairs.push((*coord, *other_coord));
        }
    }

    pairs
}

fn generate_antinodes(coord_pair: (Coordinate, Coordinate)) -> (Coordinate, Coordinate) {
    let diff_height = coord_pair.0.height - coord_pair.1.height;
    let diff_width = coord_pair.0.width-coord_pair.1.width;

    let new_c1 = Coordinate {
        height: coord_pair.0.height + diff_height,
        width: coord_pair.0.width + diff_width
    };

    let new_c2 = Coordinate{
        height: coord_pair.1.height - diff_height,
        width: coord_pair.1.width - diff_width
    };

    (new_c1, new_c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_antinodes() {
        let node1 = Coordinate{ height: 1, width: 4 };
        let node2 = Coordinate{ height: 3, width: 2 };

        let want_node_1 = Coordinate{ height: -1, width: 6 };
        let want_node_2 = Coordinate{ height: 5, width: 0 };

        assert_eq!((want_node_1, want_node_2), generate_antinodes((node1, node2)));
    }

    #[test]
    fn test_generate_all_node_pairs() {
        let empty_vec:Vec<(Coordinate, Coordinate)> = vec![];
        let node1 = Coordinate{ height: 1, width: 4 };
        assert_eq!(empty_vec, generate_all_node_pairs(vec![node1]));

        let node2 = Coordinate{ height: 3, width: 2 };
        let one_pair_vec: Vec<(Coordinate, Coordinate)> = vec![(node1, node2)];
        assert_eq!(one_pair_vec, generate_all_node_pairs(vec![node1, node2]));

        let node3 = Coordinate{ height: 1, width: 2 };
        let three_pair_vec: Vec<(Coordinate, Coordinate)> = vec![(node1, node2), (node1, node3), (node2, node3)];
        assert_eq!(three_pair_vec, generate_all_node_pairs(vec![node1, node2, node3]));
    }
}
