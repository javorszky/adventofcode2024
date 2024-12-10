mod part1;
mod part2;

pub fn solve_part1_example() -> u64 {
    part1::solve_swap(include_str!("../example.txt"))
}

pub fn solve_part1() -> u64 {
    part1::solve_swap(include_str!("../input.txt"))
}

pub fn solve_part2_example() -> u64 {
    part2::solve(include_str!("../example.txt"))
}

pub fn solve_part2() -> u64 {
    part2::solve(include_str!("../input.txt"))
}

