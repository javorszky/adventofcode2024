mod part1;

pub fn solve_part1_example() -> u64 {
    part1::solve_swap(include_str!("../example.txt"))
}

pub fn solve_part1() -> u64 {

    part1::solve_swap(include_str!("../input.txt"))
}