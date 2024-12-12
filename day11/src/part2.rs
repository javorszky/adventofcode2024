use std::collections::HashMap;

pub(crate) fn solve(input: &str) -> u64 {
    let numbers = input
        .split_whitespace()
        .map(|chunk| chunk.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut sum = 0;
    let mut memo: HashMap<(u64, usize), u64> = HashMap::new();

    for n in numbers {
        sum += count(n, 75, &mut memo)
    }

    println!("length of memo is {}", memo.len());

    sum
}

pub(crate) fn count(number: u64, steps: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if memo.contains_key(&(number, steps)) {
        return memo[&(number, steps)];
    }

    // we have reached the end of this branch, there is only one number, and it's the one we're
    // looking at, except we don't care what the number is
    if steps == 0 {
        return 1;
    }

    let number_as_string = format!("{:?}", number);
    let res;

    if number == 0 {
        res = count(1, steps - 1, memo);
    } else if number_as_string.len() % 2 == 0 {
        let left = number_as_string[..number_as_string.len()/2].parse::<u64>().unwrap();
        let right = number_as_string[number_as_string.len()/2..].parse::<u64>().unwrap();

        res = count(left, steps - 1, memo) + count(right, steps - 1, memo);
    } else {
        res = count(number * 2024, steps - 1, memo);
    }

    memo.insert((number, steps), res);

    res
}