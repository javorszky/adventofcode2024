use crate::{Line};

#[derive(Debug)]
enum Operators {
    Plus,
    Product
}

pub(crate) fn solve(data: &str) -> i64 {
    let lines: Vec<Line> = data.trim().lines().map(|x| -> Line {
        let sides = x.split(":").collect::<Vec<&str>>();

        Line {
            target: sides[0].parse::<i64>().unwrap(),
            parts: sides[1].trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
        }
    }).collect();

    lines
        .into_iter()
        .filter(|l| is_valid(l))
        .fold(0, |acc, line| {
            acc + line.target
        })
}

fn is_valid(l: &Line) -> bool {
    // dumb, use a bitmask to denote whether we're adding or multiplying
    let len = l.parts.len();
    let binary_end = 2u32.pow((len-1) as u32)-1; // this is for the mask

    'outer: for mask in 0..=binary_end {
        let mut cases:Vec<Operators> = Vec::new();

        for test in 0..len-1 {
            if mask & (1<<test) == 0 {
                cases.push(Operators::Plus);
            } else {
                cases.push(Operators::Product);
            }
        }

        let mut total = l.parts[0];
        for i in 1..l.parts.len() {
            total = operate(total, &cases[i-1], l.parts[i]);
            if total > l.target {
                continue 'outer;
            }
        }

        if total == l.target {
            return true;
        }
    }

    false
}

fn operate(left_number: i64, operator: &Operators, right_number: i64) -> i64 {
    match operator {
        Operators::Plus => {
            left_number + right_number
        }
        Operators::Product => {
            left_number * right_number
        }
    }
}