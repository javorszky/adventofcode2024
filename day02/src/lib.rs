use std::cmp::Ordering;
use crate::Direction::{Decreasing, Increasing};


pub struct Day02 {
    list: Vec<Vec<i32>>
}

enum Direction {
    Increasing,
    Decreasing,
}

impl Day02 {
    pub fn new() -> Day02 {
        let input = include_str!("../input.txt");

        let lines = input.lines().collect::<Vec<&str>>();

        let vex = lines.iter().map(|line: &&str| {
            line.split_whitespace().map(|word| word.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        }).collect();

        Day02 { list: vex }
    }

    pub fn part1(&self) -> String {
        let mut safe_list = 0;

        'outer: for l in &self.list {
            // set the direction to increasing by default

            let dir: Direction = match l[0].cmp(&l[1]) {
                Ordering::Equal => {
                    // if the first two are the same, it's unsafe, skip the loop and go next
                    continue;
                },
                Ordering::Greater => {
                    // if the first is bigger than the next, it's a decreasing
                    Decreasing
                }
                Ordering::Less => {
                    Increasing
                }
            };

            match dir {
                Increasing => {
                    for (i, _x) in l.iter().enumerate() {
                        if i == 0 {
                            continue;
                        }

                        // 7 9
                        if l[i] - l[i-1] < 1 {
                            // println!("we're doing increasing on {:?}, but {} and {} were the wrong way around, so skipping", l, l[i-1], l[i]);
                            // this means either same or decreasing, so break
                            continue 'outer;
                        } else if  l[i] - l[i-1] > 3 {
                            // println!("we're doing increasing on {:?}, but {} and {} were too far apart, so skipping", l, l[i-1], l[i]);
                            continue 'outer;
                        }
                    }

                    safe_list += 1;
                }
                Decreasing => {
                    for (i, _x) in l.iter().enumerate() {
                        if i == 0 {
                            continue;
                        }

                        // 9 7
                        if -l[i] + l[i-1] < 1 {
                            // println!("we're doing decreasing on {:?}, but {} and {} were the wrong way around, so skipping", l, l[i-1], l[i]);
                            // this means either same or decreasing, so break
                            continue 'outer;
                        } else if  -l[i] + l[i-1] > 3 {
                            // println!("we're doing decreasing on {:?}, but {} and {} were too far apart, so skipping", l, l[i-1], l[i]);
                            continue 'outer;
                        }
                    }

                    safe_list += 1;
                }
            }
        }

        format!("The number of safe reports is {}", safe_list)
    }
}


impl Default for Day02 {
    fn default() -> Self {
        Self::new()
    }
}
