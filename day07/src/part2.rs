use crate::Line;

#[derive(Debug, PartialEq)]
enum P2Operators {
    Add,
    Multiply,
    Concatenate
}


pub(crate) fn solve(input: &str) -> u64 {
    let lines: Vec<Line> = input.trim().lines().map(|x| -> Line {
        let sides = x.split(":").collect::<Vec<&str>>();

        Line {
            target: sides[0].parse::<u64>().unwrap(),
            parts: sides[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect()
        }
    }).collect();

    lines
        .into_iter()
        .filter(is_valid)
        .fold(0, |acc, line| {
            acc + line.target
        })
}

fn is_valid(l: &Line) -> bool {
    let gaps = l.parts.len()-1;

    'outer: for i in 0..3u32.pow(gaps as u32) {
        let variant = variants_to_base3(i as u64, gaps);

        let mut total = l.parts[0];
        for (j, _) in variant.iter().enumerate().take(l.parts.len()-1) {
            total = operate(total, &variant[j], l.parts[j+1]);
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


fn variants_to_base3(code: u64, len: usize) -> Vec<P2Operators>{
    let mut variants: Vec<P2Operators> = Vec::new();
    let mut steps = code;

    for _i in 0..len {
        let  remainder = steps.wrapping_rem(3);
        match remainder {
            0 => variants.push(P2Operators::Add),
            1 => variants.push(P2Operators::Multiply),
            2 => variants.push(P2Operators::Concatenate),
            _ => unreachable!()
        }

        steps = steps.checked_div(3).unwrap();
    }

    variants
}


fn operate(left_number: u64, operator: &P2Operators, right_number: u64) -> u64 {
    match operator {
        P2Operators::Add=> {
            left_number + right_number
        }
        P2Operators::Multiply => {
            left_number * right_number
        }
        P2Operators::Concatenate => {
            format!("{}{}", left_number, right_number).parse::<u64>().unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants_to_base3() {
        // Add is 0
        // Multiply is 1
        // Concatenate is 2
        assert_eq!(variants_to_base3(0, 1), vec![P2Operators::Add], "0 /1 should be just an add");
        assert_eq!(variants_to_base3(1, 2), vec![P2Operators::Multiply, P2Operators::Add], "1 / 2 should be a mul, add");
        assert_eq!(variants_to_base3(2, 3 ), vec![
            P2Operators::Concatenate,
            P2Operators::Add,
            P2Operators::Add
        ], "2 / 3 should be concatenate, add, add");
        assert_eq!(variants_to_base3(21, 4), vec![
            P2Operators::Add,
            P2Operators::Multiply,
            P2Operators::Concatenate,
            P2Operators::Add
        ], "21 / 4 should be just concatenate");
    }

    #[test]
    fn test_gaps_to_variants() {
        assert_eq!(gaps_to_variants(&0), 1);
        assert_eq!(gaps_to_variants(&4), 81);
    }

    #[test]
    fn test_operate() {
        assert_eq!(operate(32, &P2Operators::Add, 98), 130);
        assert_eq!(operate(32, &P2Operators::Multiply, 98), 3136);
        assert_eq!(operate(32, &P2Operators::Concatenate, 98), 3298);
    }
}