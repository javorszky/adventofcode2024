use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use crate::part1::Day14;

impl Day14 {
    fn find_easter_egg(&self) -> i32 {

        // let l = self.robots.len();

        let _ = self.find_iterations_where_robots_are_in_middle_topmost_row();
        0
    }

    fn check_for_flood_fill(&self) -> i32 {
        let mut i = 0;
        let bounds = (self.width, self.height);

        loop {
            i +=1;
            println!("\nlooping iteration {}", i);
            let mut uniques: HashSet<(i32, i32)> = HashSet::new();
            self.robots.iter().for_each(|r| {
                let c = r.move_robot(bounds, i);
                // println!("moved robot {} {} times and got here: {:?}", r, i, c);
                uniques.insert(c);
            });

            if flood_fill_all(uniques) {

                break;
            }

            //
            // if l == uniques.len() {
            //     if flood_fill_all(uniques) {
            //         break;
            //     }
            // }
        }

        println!("At iteration {} we found uniques", i);

        i
    }

    fn find_iterations_where_robots_are_in_middle_topmost_row(&self) -> Vec<i32> {
        let bounds = (self.width, self.height);
        let mut i = 0;
        let mut seconds = 12; // we start on iteration 12, the first one that has 1 robot in the topmost row
        let mut distance = 10; // and the next one is 10 from this one
        let mut one_at_top:Vec<i32> = Vec::new();
        let mut three_on_second: Vec<i32> = Vec::new();

        loop {
            i += 1;
            let mut robots_per_vert:HashMap<i32, i32> = HashMap::new();
            let mut robot_coords: Vec<(i32, i32)> =    Vec::new();

            self.robots.iter().for_each(|r| {
                let coords = r.move_robot(bounds, seconds);
                robot_coords.push(coords);
                *robots_per_vert.entry(coords.1).or_default() += 1;
            });

            // println!("loop {}, iteration {}, distance {}", i, iteration, distance);



            // println!("robots per entry: {:?}", robots_per_vert);

            let mut foo = robots_per_vert.keys().cloned().collect::<Vec<_>>();
            foo.sort();

            let first_key = foo[0];
            let second_key = foo[1];

            // println!("there are {} robots on the topmost key at {}", robots_per_vert.get(first_key).unwrap(), first_key);

            if robots_per_vert.get(&first_key).unwrap() == &1 {
                one_at_top.push(seconds);
                // visualise(iteration, bounds, robot_coords);
            }

            if robots_per_vert.get(&second_key).unwrap() == &3 {
                three_on_second.push(seconds);
            }

            seconds += distance;
            distance = generator(distance);


            if i > 30000 {
                break;
            }
            //
            // for r in self.robots.iter() {
            //     println!("\n------------------------\nrobot we're looking at is {}", r);
            //     println!("we need mod height {} to be 0", self.height);
            //
            //     let mut iterations_where_zero: Vec<i32> = Vec::new();
            //     let mut i = 0;
            //     loop {
            //         if r.move_robot(bounds, i).1 == 0 {
            //             iterations_where_zero.push(i);
            //         }
            //
            //         i += 1;
            //
            //         if i > 320 {
            //             break;
            //         }
            //     }
            //
            //     let mut previous = 0;
            //
            //     for j in iterations_where_zero.iter() {
            //         println!("thingy is zero vertically on iteration {}, {} since last", j, j - previous);
            //         previous = *j;
            //     }
            // }


            // we need iterations where the result is divisible by height
            // r.start_vertical + i * r.move_vertical = 100x

        }

        let mut previous = 0;
        for l in three_on_second {
            println!("at seconds {} there are three on second row, a distance of {} from the previous",
            l, l-previous);
        }
        // for l in one_at_top {
        //     // this yields a
        //     // println!("iteration {} has one at top, {} since last", l, l-previous);
        //     previous = l;
        // }


        vec![]
    }
}

fn flood_fill_all(coords: HashSet<(i32, i32)>) -> bool {
    // println!("-- checking flood fill for coords {:?}", coords);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let first = coords.iter().next().unwrap();
    // println!("-- FIRST: {:?}", first);
    visit(&coords, &mut visited, first);

    coords.len() == visited.len()
}

fn visit(map: &HashSet<(i32, i32)>, visited: &mut HashSet<(i32, i32)>, coord: &(i32, i32)) {
    // println!("-- ++ recursive visited: {:?}", coord);
    if !map.contains(coord) {
        // println!("-- ++ __ coord is not in the map");
        return;
    } // if coordinate is not in the map, return
    if visited.contains(coord) {
        // println!("-- ++ __ coord is already in the visited");
        return;
    } // if coordinate is in the visited, we've been here, ret

    // println!("-- ++ __ inserting coord into visited");
    visited.insert(*coord);
    for c in gen_neighbours(coord) {
        // println!("-- ++ __ checking neighbour {:?} of {:?}", c, coord);
        visit(map, visited, &c);
    }
}

fn gen_neighbours(coord: &(i32, i32)) -> [(i32, i32); 4] {
    [
        (coord.0-1, coord.1),
        (coord.0+1, coord.1),
        (coord.0, coord.1-1),
        (coord.0, coord.1+1)
    ]
}

pub(crate) fn solve(input: &str, width: i32, height: i32) -> i32 {
    let robots: Vec<crate::part1::Robot> = input.trim().lines().map(|line| crate::part1::Robot::try_from(line).unwrap()).collect();
    let d14 = Day14::new(width, height, 0, robots);

    d14.find_easter_egg()
}

fn visualise(iteration: i32, bounds: (i32, i32), input: Vec<(i32, i32)>) {
    let mut s = String::from("Iteration ") + &iteration.to_string() + "\n";
    for vertical in 0..=bounds.0 {
        for horizontal in 0..=bounds.1 {
            match input.contains(&(horizontal, vertical)) {
                true => {s.push('#');}
                false => {s.push('.');}
            }
        }
        s.push('\n');
    }

    let mut file = File::create(iteration.to_string() + "tree.txt").unwrap();
    file.write_all(s.as_bytes()).unwrap();

    // println!("{}", s);
}

fn generator(previous: i32) -> i32 {
    match previous {
        10 => {46}
        46 => {47}
        47 => {10}
        _ => {103}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill_all() {
        let input = "p=0,0 v=1,-1\n\
        p=3,1 v=-2,-1\n\
        p=2,2 v=-1,-1\n\
        p=0,3 v=1,3\n";

        assert_eq!(solve(input, 4, 4), 1);

    }
}