mod part1;
mod part2;

pub fn solve_part1_example() -> i32 {
    part1::solve(include_str!("../example.txt"))
}

pub fn solve_part1() -> i32 {
    part1::solve(include_str!("../input.txt"))
}

pub fn solve_part2_example() -> i32 {
    part2::solve(include_str!("../example.txt"))
}

pub fn solve_part2() -> i32 { 0 }