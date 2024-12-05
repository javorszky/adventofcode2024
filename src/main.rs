
fn main() {

    println!("You're reading the Advent of Code 2024 solutions by Gabor Javorszky.");
    println!("Find the source code at https://github.com/javorszky/adventofcode2024.");

    // Day 01
    let d1 = day01::Day01::new();
    println!("\nDay 1 part 1: {}.",  d1.solve());
    println!("Day 1 part 2: {}.",  d1.solve_two());

    // Day 02
    let d2 = day02::Day02::new();
    println!("\nDay 2 part 1: {}.", d2.part1());
    let (_, result) = d2.part2();
    println!("Day 2 part 2: {}.", result);

    // Day 03
    let d3 = day03::Day03::new();
    println!("\nDay 3 part 1: sum of valid mul products is {}.", d3.part1());

    println!("Day 3 part 2: sum of valid and enabled mul products is {}.", d3.part2());

    // Day 4
    println!("\nDay 4 part 1: number of XMAS words: {}.", day04::part1());
    println!("Day 4 part 2: number of pair of MAS words in diagonal X shape: {}.", day04::part2());

    // Day 5
    println!("\nDay 5 part 1: sum of middle numbers of valid lines is {}.", day05::part1());
    println!("Day 5 part 2: after fixing the invalid lines, their middles is {}.", day05::part2());
}
