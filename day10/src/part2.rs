use crate::part1::Day10;

pub(crate) fn solve(input: &str) -> usize {
    let d10 = Day10::new(input);

    let trails = d10.find_trails();

    trails.len()
}