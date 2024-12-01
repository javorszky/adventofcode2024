
fn main() {

    println!("Hello world! You're reading the Advent of code solutions by Gabor Javorszky.\n");

    // Day 01
    let d1 = day01::Day01::new();
    println!("Day 1 part 1: {}",  d1.solve());
    println!("Day 1 part 2: {}\n",  d1.solve_two());


}
