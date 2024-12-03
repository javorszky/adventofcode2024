use regex::Regex;

const REGEX_STRING: &str = r"mul\(\d{1,3},\d{1,3}\)";

struct Mul {
    a: i32,
    b: i32
}

impl Mul {
    fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    pub fn product(&self) -> i32 {
        self.a * self.b
    }
}

pub struct Day03 {
    muls: Vec<Mul>,
}

impl Day03 {
    pub fn new() -> Day03 {
        Day03{
            muls: Regex::new(REGEX_STRING).unwrap()
                .find_iter(include_str!("../input.txt"))
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
                .iter()
                .map(|m|  parse_mul(m))
                .collect::<Vec<Mul>>()
        }
    }
}

impl Day03 {
    pub fn part1(&self) -> i32 {
        let mut sum = 0;

        self.muls.iter().for_each(|m| sum += m.product());

        sum
    }

}
impl Default for Day03 {
    fn default() -> Day03 {
        Day03::new()
    }
}

fn parse_mul(input: &str) -> Mul {
    let numbers = input
        .strip_prefix("mul(").unwrap()
        .strip_suffix(")").unwrap()
        .split(",").map(|m|
        m.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Mul::new(numbers[0], numbers[1])
}
