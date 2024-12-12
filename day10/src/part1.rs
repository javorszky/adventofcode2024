use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub (crate) struct Coordinate {
    height: usize,
    width: usize,
}

impl Coordinate {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }

    pub fn neighbours(&self) -> Vec<Self> {
        let mut n = Vec::new();
        match self.height.cmp(&0) {
            Ordering::Less => { panic!("coordinate should not have negative height as unsigned number!") }
            Ordering::Equal => {
                n.push(Self::new(self.height+1, self.width));
            }
            Ordering::Greater => {
                n.push(Self::new(self.height+1, self.width));
                n.push(Self::new(self.height-1, self.width));
            }
        }

        match self.width.cmp(&0) {
            Ordering::Less => { panic!("coordinate should not have negative width as unsigned number!") }
            Ordering::Equal => {
                n.push(Self::new(self.height, self.width+1));
            }
            Ordering::Greater => {
                n.push(Self::new(self.height, self.width-1));
                n.push(Self::new(self.height, self.width+1));
            }
        }

        n
    }
}


pub (crate) struct Day10 {
    map: HashMap<Coordinate, u8>,
    trailheads: Vec<Coordinate>
}

impl Day10 {
    pub fn new(data: &str) -> Self {
        let mut map: HashMap<Coordinate, u8> = HashMap::new();
        let mut trailheads = Vec::new();

        for (height, line) in data.trim().lines().enumerate() {
            for (width, character) in line.trim().chars().enumerate() {
                let h = u8::try_from(character.to_digit(10).unwrap()).unwrap();
                let c = Coordinate{height, width};
                map.insert(c, h);

                if h == 0 {
                    trailheads.push(c);
                }
            }
        }

        Day10 { map, trailheads }
    }

    pub(crate) fn find_trails(&self) -> Vec<Vec<Coordinate>> {
        let mut trails = Vec::new();

        for &head in self.trailheads.iter() {
            let mut new_trails = self.trail_next(head, vec![]);

            trails.append(&mut new_trails);
        }

        trails
    }

    pub(crate) fn find_score(&self) -> usize {
        let mut score: usize = 0;

        for &head in self.trailheads.iter() {
            let new_trails = self.trail_next(head, vec![]);

            score += find_nines(new_trails);
        }

        score
    }

    fn trail_next(&self, current: Coordinate, mut trail_so_far: Vec<Coordinate>) -> Vec<Vec<Coordinate>> {
        let v = self.map.get(&current).unwrap_or(&10);
        trail_so_far.push(current);

        // println!("== Starting trail_next block with current {} @ {:?}  ==", v, current);

        match v.cmp(&9) {
            Ordering::Equal => {
                /* we have found a trail */
                // println!("== Ending trail_next block as we found a full trail\n{:?}\n==\n", trail_so_far);
                vec![trail_so_far]
            }
            Ordering::Greater => {
                /* this is out of map */
                // println!("== Ending trail_next as this coordinate is out of map, returning empty ==");
                vec![]
            }
            Ordering::Less => {
                let mut new_trails: Vec<Vec<Coordinate>> = Vec::new();
                // println!("  -- for each neighbours:");
                for c in current.neighbours() {
                    let n = self.map.get(&c).unwrap_or(&10);
                    // println!("    -- value {} @ {:?}", n, c);

                    match n.cmp(&(v + 1)) {
                        Ordering::Equal => {
                            // println!("    -- value {} is one more than {}, generating new trails from here", n, v);
                            for found_trail in self.trail_next(c, trail_so_far.clone()).iter() {
                                // print!("   == Ending trail, found new ones! ==");
                                new_trails.push(found_trail.clone());
                            }
                        }
                        _ => { /* do nothing here */ }
                    }
                }

                // println!("new trails:\n{}{:?}", "->".repeat(u32::from(*v) as usize), new_trails);

                new_trails
            }
        }
    }
}

pub(crate) fn solve(data: &str) -> usize {
    let d10 = Day10::new(data);

    d10.find_score()
}

fn find_nines(trails: Vec<Vec<Coordinate>>) -> usize {
    let s:HashSet<Coordinate> = HashSet::from_iter(trails.iter().map(|v| *v.last().unwrap()));

    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate() {
        let c = Coordinate::new(5, 3);
        assert_eq!(c.neighbours().len(), 4);
        assert_eq!(c.neighbours(), vec![
            Coordinate::new(6, 3),
            Coordinate::new(4, 3),
            Coordinate::new(5, 2),
            Coordinate::new(5, 4),
        ]);

        let c0 = Coordinate::new(0, 0);
        assert_eq!(c0.neighbours().len(), 2);
        assert_eq!(c0.neighbours(), vec![
            Coordinate::new(1, 0),
            Coordinate::new(0, 1),
        ]);
    }

    #[test]
    fn test_find_trails() {
        let input = "0123456789";
        let d = Day10::new(input);

        assert_eq!(d.find_trails(), vec![vec![
            Coordinate::new(0, 0),
            Coordinate::new(0, 1),
            Coordinate::new(0, 2),
            Coordinate::new(0, 3),
            Coordinate::new(0, 4),
            Coordinate::new(0, 5),
            Coordinate::new(0, 6),
            Coordinate::new(0, 7),
            Coordinate::new(0, 8),
            Coordinate::new(0, 9),
        ]]);
    }

    #[test]
    fn test_find_two_trails() {
        let input = "\n\
        0123456789\n\
        8343522222\n\
        1111678922";

        let d = Day10::new(input);

        assert_eq!(d.find_trails(), vec![
            vec![
                Coordinate::new(0, 0), // 0
                Coordinate::new(0, 1), // 1
                Coordinate::new(0, 2), // 2
                Coordinate::new(0, 3), // 3
                Coordinate::new(0, 4), // 4
                Coordinate::new(1, 4), // 5
                Coordinate::new(2, 4),
                Coordinate::new(2, 5),
                Coordinate::new(2, 6),
                Coordinate::new(2, 7),
            ],
            vec![
                Coordinate::new(0, 0), // 0
                Coordinate::new(0, 1), // 1
                Coordinate::new(0, 2), // 2
                Coordinate::new(0, 3), // 3
                Coordinate::new(0, 4), // 4
                Coordinate::new(0, 5), // 5
                Coordinate::new(0, 6),
                Coordinate::new(0, 7),
                Coordinate::new(0, 8),
                Coordinate::new(0, 9),
            ],

        ]);
    }

    #[test]
    fn test_find_multiple_trails() {
        let input = "\n\
        9934522222\n\
        0129678922\n\
        9934522222";

        let d = Day10::new(input);

        assert_eq!(d.find_trails(), vec![
            vec![
                Coordinate::new(1, 0), // 0
                Coordinate::new(1, 1), // 1
                Coordinate::new(1, 2), // 2
                Coordinate::new(2, 2), // 3
                Coordinate::new(2, 3), // 4
                Coordinate::new(2, 4), // 5
                Coordinate::new(1, 4), // 6
                Coordinate::new(1, 5),
                Coordinate::new(1, 6),
                Coordinate::new(1, 7),
            ],
            vec![
                Coordinate::new(1, 0), // 0
                Coordinate::new(1, 1), // 1
                Coordinate::new(1, 2), // 2
                Coordinate::new(0, 2), // 3
                Coordinate::new(0, 3), // 4
                Coordinate::new(0, 4), // 5
                Coordinate::new(1, 4), // 6
                Coordinate::new(1, 5),
                Coordinate::new(1, 6),
                Coordinate::new(1, 7),
            ],
        ]);
    }

    #[test]
    fn test_score() {
        println!("testing score");

        let input = "1066911\n\
2666866\n\
3111711\n\
4567654\n\
1118113\n\
1119662\n\
3333301";

        let d = Day10::new(input);
        let trails = d.find_trails();

        assert_eq!(trails.len(), 3);
    }
}