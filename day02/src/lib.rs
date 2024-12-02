use crate::Direction::{Decreasing, Increasing};
use std::cmp::Ordering;

pub struct Day02 {
    list: Vec<Vec<i32>>,
}

enum Direction {
    Increasing,
    Decreasing,
}

impl Day02 {
    pub fn new() -> Day02 {
        let input = include_str!("../input.txt");

        let lines = input.lines().collect::<Vec<&str>>();

        let vex = lines
            .iter()
            .map(|line: &&str| {
                line.split_whitespace()
                    .map(|word| word.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

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
                }
                Ordering::Greater => {
                    // if the first is bigger than the next, it's a decreasing
                    Decreasing
                }
                Ordering::Less => Increasing,
            };

            match dir {
                Increasing => {
                    for (i, _x) in l.iter().enumerate() {
                        if i == 0 {
                            continue;
                        }

                        // 7 9
                        if l[i] - l[i - 1] < 1 {
                            // println!("we're doing increasing on {:?}, but {} and {} were the wrong way around, so skipping", l, l[i-1], l[i]);
                            // this means either same or decreasing, so break
                            continue 'outer;
                        } else if l[i] - l[i - 1] > 3 {
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
                        if -l[i] + l[i - 1] < 1 {
                            // println!("we're doing decreasing on {:?}, but {} and {} were the wrong way around, so skipping", l, l[i-1], l[i]);
                            // this means either same or decreasing, so break
                            continue 'outer;
                        } else if -l[i] + l[i - 1] > 3 {
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

    pub fn part2(&self) -> (i32, String) {
        let mut safe_list = 0;

        // let boo = [vec![1, 2, 3, 2, 3, 4, 5],];

        for l in &self.list {
        // for l in &boo {
            let inc = check_increasing(l.clone());
            let dec = check_decreasing(l.clone());

            if inc || dec {
                // we have found a list that is safe without modification!
                safe_list += 1;
                continue;
            }

            let vecs = generate_vecs(l.clone());
            for v in vecs {
                let inc = check_increasing(v.clone());
                let dec = check_decreasing(v.clone());

                if inc || dec {
                    // we have found a list that works, no need to check the rest
                    safe_list += 1;
                    break;
                }
            }
        }

        (safe_list, format!("The number of safe reports with the dampener is {}", safe_list))
    }
}

impl Default for Day02 {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_vecs(list: Vec<i32>) -> Vec<Vec<i32>> {
    let len = list.len();
    let mut vex: Vec<Vec<i32>> = Vec::new();

    for s in 0..len {
        let mut local_vec: Vec<i32> = Vec::new();
        for (i, n) in list.iter().enumerate() {
            if s == i {
                continue;
            }

            local_vec.push(*n);
        }

        vex.push(local_vec);
    }

    vex
}

fn check_increasing(list: Vec<i32>) -> bool {
    for (i, &x) in list.iter().enumerate() {
        // skip the first one so we can guarantee that there's a "previous"
        if i == 0 {
            continue;
        }

        let current = x;
        let previous = list[i - 1];

        if current <= previous {
            return false
        }

        if current - previous > 3 {
            return false
        }
    }

    true
}

fn check_decreasing(list: Vec<i32>) -> bool {
    for (i, &x) in list.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let current = x;
        let previous = list[i - 1];

        if current >= previous {
            return false;
        }

        if previous - current > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_increasing_good() {
        assert_eq!(true, check_increasing(vec![1, 2, 3, 5, 8]))
    }

    #[test]
    fn test_check_increasing_eq_bad() {
        assert_eq!(false, check_increasing(vec![1, 2, 3, 3, 4, 5]))
    }

    #[test]
    fn test_check_increasing_lower() {
        assert_eq!(false, check_increasing(vec![1, 2, 3, 4, 3]))
    }

    #[test]
    fn test_check_increasing_gap() {
        assert_eq!(false, check_increasing(vec![1, 2, 3, 4, 8]))
    }

    #[test]
    fn test_check_decreasing_good() {
        assert_eq!(true, check_decreasing(vec![8, 5, 3, 2, 1]))
    }

    #[test]
    fn test_check_decreasing_eq_bad() {
        assert_eq!(false, check_decreasing(vec![5, 4, 3, 3, 2, 1]))
    }

    #[test]
    fn test_check_decreasing_higher_bad() {
        assert_eq!(false, check_decreasing(vec![5, 4, 3, 4, 3, 2, 1]))
    }

    #[test]
    fn test_check_decreasing_gap_bad() {
        assert_eq!(false, check_decreasing(vec![8, 4, 3, 2, 1]))
    }

    #[test]
    fn test_generate_vecs() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let vecs = generate_vecs(input);

        assert_eq!(vecs.len(), 8);
        assert_eq!(vecs[0], vec![2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(vecs[1], vec![1, 3, 4, 5, 6, 7, 8]);
        assert_eq!(vecs[2], vec![1, 2, 4, 5, 6, 7, 8]);
        assert_eq!(vecs[3], vec![1, 2, 3, 5, 6, 7, 8]);
        assert_eq!(vecs[4], vec![1, 2, 3, 4, 6, 7, 8]);
        assert_eq!(vecs[5], vec![1, 2, 3, 4, 5, 7, 8]);
        assert_eq!(vecs[6], vec![1, 2, 3, 4, 5, 6, 8]);
        assert_eq!(vecs[7], vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_part2_increasing_no_fuse() {
        let d = Day02 {
            list: vec![ vec![1, 2, 3, 4, 5, 6, 7], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 1);
    }

    #[test]
    fn test_part2_decreasing_no_fuse() {
        let d = Day02 {
            list: vec![ vec![6, 5, 4, 3, 2, 1], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 1);
    }

    #[test]
    fn test_part2_increasing_fused_eq() {
        let d = Day02 {
            list: vec![ vec![1, 2, 3, 4, 4, 5, 6, 7] ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 1);
    }

    #[test]
    fn test_part2_decreasing_fused_eq() {
        let d = Day02 {
            list: vec![ vec![6, 5, 4, 3, 3, 2, 1], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 1);
    }

    #[test]
    fn test_part2_increasing_fused_dip_higher() {
        let d = Day02 {
            list: vec![ vec![ 1, 2, 3, 4, 3, 5, 6, 7], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 1);
    }

    #[test]
    fn test_part2_decreasing_fused_peak_lower() {
        let d = Day02 {
            list: vec![ vec![7, 6, 5, 4, 5, 3, 2, 1], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 1);
    }

    #[test]
    fn test_part2_increasing_dip_lower_than_threshold() {
        let d = Day02 {
            list: vec![ vec![1, 2, 3, 4, 5, 3, 4, 6, 7], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_decreasing_peak_higher_than_threshold() {
        let d = Day02 {
            list: vec![ vec![7, 6, 5, 4, 3, 6, 4, 2, 1], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_increasing_fused_gap() {
        let d = Day02 {
            list: vec![ vec![1, 2, 3, 4, 8, 9], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_increasing_two_gaps() {
        let d = Day02 {
            list: vec![ vec![1, 2, 6, 7, 11, 12], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_decreasing_big_gap() {
        let d = Day02 {
            list: vec![ vec![8, 7, 6, 2, 1], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_decreasing_two_big_gaps() {
        let d = Day02 {
            list: vec![vec![13, 12, 8, 7, 6, 2, 1], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_increasing_two_eq() {
        let d = Day02 {
            list: vec![vec![1, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_decreasing_two_eq() {
        let d = Day02 {
            list: vec![ vec![9, 8, 7, 7, 6, 5, 4, 4, 3, 2], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_increasing_same_three() {
        let d = Day02 {
            list: vec![ vec![1, 2, 3, 3, 3, 4, 5, 6, 7], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_decreasing_same_three() {
        let d = Day02 {
            list: vec![ vec![8, 7, 6, 5, 5, 5, 4, 3, 2],]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_increasing_dip_eq() {
        let d = Day02 {
            list: vec![  vec![1, 2, 3, 4, 3, 4, 5, 6],]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }

    #[test]
    fn test_part2_decreasing_dip_eq() {
        let d = Day02 {
            list: vec![ vec![8, 7, 6, 5, 6, 5, 4, 3], ]
        };

        let (valid, _) = d.part2();

        assert_eq!(valid, 0);
    }
}