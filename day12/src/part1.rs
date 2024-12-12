use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Coordinate {
    horizontal: i32,
    vertical: i32,
}

impl Coordinate {
    fn new(horizontal: i32, vertical: i32) -> Self {
        Self { horizontal, vertical }
    }

    fn neighbours(&self) -> Vec<Self> {
        vec![
            Self::new(self.horizontal-1, self.vertical),
            Self::new(self.horizontal+1, self.vertical),
            Self::new(self.horizontal, self.vertical-1),
            Self::new(self.horizontal, self.vertical+1),
        ]
    }
}

struct Map {
    map: HashMap<Coordinate, String>
}

#[derive(Debug)]
struct Region {
    plots: HashSet<Coordinate>
}

impl Region {
    fn new(plots: HashSet<Coordinate>) -> Self {
        Self { plots }
    }

    fn area(&self) -> i32 {
        self.plots.len() as i32
    }

    fn perimeter(&self) -> i32 {
        let mut visited: HashSet<Coordinate> = HashSet::new();

        let mut perimeter = 0;

        for c in self.plots.iter() {
            if visited.contains(c) {
                continue;
            }

            for n in c.neighbours() {
                if !self.plots.contains(&n) {
                    perimeter += 1;
                }
            }
            // count out of set neighbours
        }

        perimeter
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
        let region = flood_fill(&map, *coord, map.get(coord).unwrap(), &mut visited);

        regions.push(Region { plots: region });
    }

    let regions: Vec<&Region> = regions.iter().filter(|&r| !r.plots.is_empty()).collect();

    let mut price: i32 = 0;
    for region in regions.iter() {
        price += region.perimeter() * region.area();
    }

    price as u32
}

fn flood_fill(map: &HashMap<Coordinate, String>, origin: Coordinate, plot_type: &String, visited: &mut HashSet<Coordinate>) -> HashSet<Coordinate> {
    let unknown: &String = &"1".to_string();
    // println!("flood fill on coordinate {:?}", origin);
    let mut local_set = HashSet::new();

    if visited.contains(&origin) {
        // println!("-- we've been here previously, returning an empty set");
        return local_set;
    }

    let plot_at_coord = map.get(&origin).unwrap_or(unknown);

    if plot_at_coord != plot_type {
        // println!("-- we were looking for plot type {}, but this {} was different",plot_type, plot_at_coord);
        return local_set
    }

    // println!("-- inserting into visited and local set");

    // we're on a plot that's the same as the region we're trying to find
    local_set.insert(origin);
    visited.insert(origin);

    // println!("-- checking neighbours");
    for neighbour in origin.neighbours() {
        for p in flood_fill(map, neighbour, plot_type, visited) {
            // println!("-- -- inserting coordinate {:?} into local set from neighbour {:?}", p, neighbour);
            local_set.insert(p);
        }
    }

    local_set
}