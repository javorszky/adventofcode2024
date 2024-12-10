mod part1;

pub fn solve_part1_example() -> usize {
    let input = include_str!("../example.txt");

    part1::solve(input)
}

pub fn solve_part1() -> usize {
    let input = include_str!("../input.txt");

    part1::solve(input)
}

