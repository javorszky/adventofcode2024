mod part1;

const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[derive(Debug)]
struct Line {
    target: i64,
    parts: Vec<i64>,
}

pub fn solve_part1_example() -> i64 {
    part1::solve(EXAMPLE)
}

pub fn solve_part1() -> i64 {
    part1::solve(include_str!("../input.txt"))
}