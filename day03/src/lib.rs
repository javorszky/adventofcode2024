use regex::{Match, Regex};

const MUL_REGEX_STRING: &str = r"mul\(\d{1,3},\d{1,3}\)";
const DO_REGEX_STRING: &str = r"(do\(\))";
const DONT_REGEX_STRING: &str = r"(don't\(\))";

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
    input: &'static str,
    all_muls: Vec<Mul>,
    dos: Vec<(usize, usize)>,
    donts: Vec<(usize, usize)>,
}

impl Day03 {
    pub fn new() -> Day03 {
        let contents = include_str!("../input.txt");

        Day03 {
            all_muls: Regex::new(MUL_REGEX_STRING)
                .unwrap()
                .find_iter(contents)
                .map(|m| { parse_mul(&m) })
                .collect::<Vec<Mul>>(),

            dos: Regex::new(DO_REGEX_STRING)
                .unwrap()
                .find_iter(contents)
                .map(|m| (m.start(), m.end()))
                .collect::<Vec<(usize, usize)>>(),

            donts: Regex::new(DONT_REGEX_STRING)
                .unwrap()
                .find_iter(contents)
                .map(|m| (m.start(), m.end()))
                .collect::<Vec<(usize, usize)>>(),

            input: contents,
        }
    }

    pub fn part1(&self) -> i32 {
        let mut sum = 0;

        self.all_muls.iter().for_each(|m| sum += m.product());

        sum
    }

    pub fn part2(&self) -> i32 {
        let enabled = self.get_enabled_only();
        let mut sum = 0;

        Regex::new(MUL_REGEX_STRING)
            .unwrap()
            .find_iter(&enabled)
            .map(|m| { parse_mul(&m) })
            .collect::<Vec<Mul>>()
            .iter().for_each(|m| sum += m.product());

        sum
    }

    fn get_enabled_only(&self) -> String {
        let mut enabled: String = "".to_owned();
        // start regex
        let re_start = Regex::new(r"(?s)^.*?don't\(\)").unwrap();
        let beginning = re_start.find(self.input).unwrap().as_str();

        enabled.push_str(beginning);

        // then the middles
        let re_mid = Regex::new(r"(?s)do\(\).*?don't\(\)").unwrap();
        let mids = re_mid.find_iter(self.input).map(|m| { m.as_str() }).collect::<Vec<&str>>();

        for l in mids {
            enabled.push_str(l);
        }

        let last_do = self.dos.last().unwrap();
        let last_dont = self.donts.last().unwrap();

        // if the last thing is not a do, then it's a don't at which point we can ignore the end of
        // the string.
        if last_do.1 > last_dont.1 {
            // the last thing we have is a do, so let's add the end from there to the end of the
            // input

            enabled.push_str(&self.input[last_do.1..]);
        }

        enabled.as_str().to_owned()
    }
}

impl Default for Day03 {
    fn default() -> Day03 {
        Day03::new()
    }
}

fn parse_mul(input: &Match) -> Mul {
    let numbers = input.as_str()
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split(",")
        .map(|m| m.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Mul::new(numbers[0], numbers[1])
}
