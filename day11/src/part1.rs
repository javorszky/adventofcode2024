#[derive(Debug, Clone, Eq, PartialEq)]
enum ParseError {
    NotEvenDigit,
    NotANumber
}

type Result<T> = std::result::Result<T, ParseError>;

pub(crate) fn blink(input: u64) -> Vec<u64> {
    if input == 0 {
        return vec![1]
    }

    if let Ok(nums) = half_number(input) {
        return vec![
            nums.0,
            nums.1,
        ]
    }

    vec![input * 2024]
}

fn half_number(input: u64) -> Result<(u64, u64)> {
    let number_as_string = format!("{}", input);
    let l = number_as_string.len();
    if l % 2 != 0 {
        return Err(ParseError::NotEvenDigit)
    }

    let fh = &number_as_string[..l/2].parse::<u64>().or(Err(ParseError::NotANumber))?;
    let sh = &number_as_string[l/2..].parse::<u64>().or(Err(ParseError::NotANumber))?;

    Ok((*fh, *sh))
}

pub(crate) fn solve(input: &str) -> usize {
    let mut numbers = input
        .split_whitespace()
        .map(|chunk| chunk.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // let mut precomputed: HashMap<u64, Vec<u64>> = HashMap::new();

    for _i in 0..25 {
        let mut new_numbers = Vec::new();
        for num in numbers.iter() {
            new_numbers.append(&mut blink(*num));
        }

        numbers = new_numbers;
    }

    numbers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_number() {
        assert_eq!(half_number(0), Err(ParseError::NotEvenDigit));
        assert_eq!(half_number(10), Ok((1, 0)));
        assert_eq!(half_number(1000), Ok((10, 0)));
        assert_eq!(half_number(999), Err(ParseError::NotEvenDigit));
    }

    #[test]
    fn test_blink() {
        assert_eq!(blink(0), vec![1]);
        assert_eq!(blink(1), vec![2024]);
        assert_eq!(blink(11), vec![1, 1]);
        assert_eq!(blink(2000), vec![20, 0]);
    }
}