use crate::part1::{ClawMachine, Coordinate, Vector};

const COST:u64 = 10000000000000;


impl Coordinate {
    fn try_from_part2(input: &str) -> Option<Coordinate> {
        println!("hello from part 2 coordinate try from");

        let nums = crate::part1::str_to_nums(&input[7..])?;
        Some(Self::new(nums.0+COST, nums.1+COST))
    }
}


impl ClawMachine {
    fn try_from_part2(input: &str) -> Option<ClawMachine> {
        let parts = input.trim().lines().collect::<Vec<&str>>();
        let ba: Vector = Vector::try_from(parts[0])?;
        let bb: Vector = Vector::try_from(parts[1])?;
        let target = Coordinate::try_from_part2(parts[2])?;

        Some(Self::new(ba, bb, target))
    }
}

pub(crate) fn solve(input: &str) -> i32 {
    let machines:Vec<ClawMachine> = input
        .trim()
        .split("\n\n")
        .map(|block| { ClawMachine::try_from_part2(block).unwrap() }).collect();

    let mut sum = 0;

    for machine in machines {
        match machine.least_tokens() {
            None => {}
            Some(boop) => {
                sum += boop;
            }
        }
    }

    sum
}