mod part1;

pub fn solve_part1_example() -> u32 {
    part1::solve(include_str!("../example.txt"))
}

pub fn solve_part1() -> u32 {
    part1::solve(include_str!("../input.txt"))
}