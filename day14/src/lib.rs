mod part1;
mod part2;

pub fn solve_part1_example() -> i32 {
    part1::solve(include_str!("../example.txt"), 11, 7, 100)
}

pub fn solve_part1() -> i32 {
    part1::solve(include_str!("../input.txt"), 101, 103, 100)
}


pub fn solve_part2() -> i32 {
    part2::solve(include_str!("../input.txt"), 101, 103)
}