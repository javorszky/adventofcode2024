use std::collections::HashMap;
use std::vec;
use crate::{part1, part2};

pub(crate) fn solve(input: &str) -> u64 {
    let mut numbers = input
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
    let mut res = 0;

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

fn build_memoize_10_cover() -> Vec<Vec<u64>> {
    const CAP: usize =  10;

    let mut memo = Vec::new();

    for i in 0..1000u64 {
        let mut results:Vec<u64> = vec![i];

        for _ in 0..CAP {
            let mut new_numbers = Vec::new();
            for num in results.iter() {
                new_numbers.append(&mut part1::blink(*num));
            }

            results = new_numbers;
        }

        memo[i as usize]=results;
    }

    memo
}


fn build_memoize_20() -> HashMap<u64, Vec<Vec<u64>>> {
    const CAP: usize =  10;

    let mut memo = HashMap::new();
    for i in 0..1000u64 {
        let mut results:Vec<Vec<u64>> = Vec::new();
        let mut numbers = vec![i];

        for _ in 0..CAP {
            let mut new_numbers = Vec::new();
            for num in numbers.iter() {
                new_numbers.append(&mut part1::blink(*num));
            }

            results.push(new_numbers.clone());

            numbers = new_numbers;
        }

        memo.insert(i, results);
    }

    memo
}

fn ten_steps(number: u64, memo: &mut HashMap<u64, Vec<Vec<u64>>>) -> Vec<Vec<u64>> {
    let mut numbers = vec![number];
    let mut results= vec![];

    const CAP:usize  = 10;

    memo.get(&number).unwrap_or_else(|| {
        println!("{:?}", number);

        for iter in 0..CAP {
            let mut new_numbers = Vec::new();
            for num in numbers.iter() {
                let mut new_numbers = Vec::new();

                match memo.get(num) {
                    None => {
                        new_numbers.append(&mut part1::blink(*num));
                    }
                    Some(res) => {}
                }

                // new_numbers.append(&mut res);
            }

            results[iter] = new_numbers.clone();

            numbers = new_numbers;
        }

        return &results
    });

    if let Some(list) = memo.get(&number) {
        return list.clone()
    }

    results
}

fn get_from_map(number: u64, steps: &usize, memo: &HashMap<u64, Vec<Vec<u64>>>) -> Option<Vec<u64>> {
    if steps == &0 {
        return Some(vec![number]);
    }

    if steps > &10 {
        return None;
    }

    match memo.get(&number) {
        None => {
            let mut blink_slice: Vec<u64> = Vec::new();
            let nums = part1::blink(number);
            for num in nums.iter() {
                blink_slice.append(&mut get_from_map(*num, &(steps - 1), memo).unwrap());
            }

            Some(blink_slice)
        }
        Some(results) => {
            if steps <= &results.len() {
                return Some(results[*steps-1].clone())
            }

            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_from_map() {
        let mut map: HashMap<u64, Vec<Vec<u64>>> = HashMap::new();
        map.insert(2, vec![
            vec![4048],
            vec![40, 48],
            vec![4, 0, 4, 8],
            vec![8096, 1, 8096, 16192],
            vec![80, 96, 2024, 80, 96, 32772608],
            vec![8, 0, 9, 6, 20, 24, 8, 0, 9, 6, 3277, 2608],
            vec![16192, 1, 18216, 12144, 2, 0, 2, 4, 16192, 1, 18216, 12144, 32, 77, 26, 8],
            vec![32772608, 2024, 36869184, 24579456, 4048, 1, 4048, 8096, 32772608, 2024, 36869184, 24579456, 3, 2, 7, 7, 2, 6, 16192],
            vec![3277, 2608, 20, 24, 3686, 9184, 2457, 9456, 40, 48, 2024, 40, 48, 80, 96, 3277, 2608, 20, 24, 3686, 9184, 2457, 9456, 6072, 4048, 14168, 14168, 4048, 12144, 32772608],
            vec![32, 77, 26, 8, 2, 0, 2, 4, 36, 86, 91, 84, 24, 57, 94, 56, 4, 0, 4, 8, 20, 24, 4, 0, 4, 8, 8, 0, 9, 6, 32, 77, 26, 8, 2, 0, 2, 4, 36, 86, 91, 84, 24, 57, 94, 56, 60, 72, 40, 48, 28676032, 28676032, 40, 48, 24579456, 3277, 2608]
        ]);

        let mut want:Vec<u64> = vec![40, 48];
        assert_eq!(get_from_map(2, &2, &map ), Some(want));

        want = vec![8096, 1, 8096, 16192];
        assert_eq!(get_from_map(2, &4, &map ), Some(want));

        assert_eq!(get_from_map(1, &4, &map ), None);
    }

    #[test]
    fn test_odd_numbered_thing() {
        let mut start:Vec<u64> = vec![0];
        let mut results: Vec<u64> = vec![];
        for _ in 0..15 {
            start.iter().for_each(|step| {
                results.append(&mut part1::blink(*step));
            });

            println!("blink of start len ({}) is {} long", start.len(), results.len());

            start = results.clone();
        }

        assert!(true)
    }

    #[test]
    fn test_count() {
        assert_eq!(count(2, 2), 2);
    }
}