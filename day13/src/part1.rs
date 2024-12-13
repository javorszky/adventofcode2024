#[derive(Debug)]
pub(crate) struct Vector {
    horizontal: i64,
    vertical: i64
}

impl Vector {
    pub(crate) fn try_from(input: &str) -> Option<Vector> {
        let nums = str_to_nums(&input[10..])?;

        Some(Self::new(nums.0, nums.1))
    }

    fn new(horizontal: i64, vertical: i64) -> Vector {
        Self { horizontal, vertical }
    }
}

#[derive(Debug)]
pub(crate) struct Coordinate {
    horizontal: i64,
    vertical: i64,
}

impl Coordinate {
    fn try_from(input: &str) -> Option<Coordinate> {
        let nums = str_to_nums(&input[7..])?;
        Some(Self::new(nums.0, nums.1))
    }

    pub(crate) fn new(horizontal: i64, vertical: i64) -> Coordinate {
        Coordinate { horizontal, vertical }
    }
}

#[derive(Debug)]
pub(crate) struct ClawMachine {
    price_a: i64,
    price_b: i64,
    button_a: Vector,
    button_b: Vector,
    target: Coordinate
}

impl ClawMachine {
    fn try_from(input: &str) -> Option<ClawMachine> {
        let parts = input.trim().lines().collect::<Vec<&str>>();
        let ba: Vector = Vector::try_from(parts[0])?;
        let bb: Vector = Vector::try_from(parts[1])?;
        let target = Coordinate::try_from(parts[2])?;

        Some(Self::new(ba, bb, target))
    }

    pub(crate) fn new(ba: Vector, bb: Vector, target: Coordinate) -> ClawMachine {
        Self { button_a: ba, button_b: bb, target, price_a: 3, price_b: 1 }
    }

    pub(crate) fn least_tokens(&self) -> Option<i64> {
        // normalize around A
        // target.x = pressA * A.x + pressB * B.x
        // target.y = pressA * A.y + pressB * B.y
        //
        // multiply first one by B.y
        // multiply second one by B.x
        // B.y * target.x = pressA * A.x * B.y + pressB * B.x * B.y
        // B.x * target.y = pressA * A.y * B.x + pressB * B.y * B.x
        //
        // Subtract top from bottom
        // B.x * target.y - B.y * target.x = pressA * A.y * B.x - pressA * A.x * B.y
        //
        // Extract pressA
        // B.x * target.y - B.y * target.x = pressA * (Ay * Bx - Ax * By)
        //
        // Divide by (Ay*By - Ax*By)
        // (B.x * target.y - B.y * target.x) / (Ay * Bx - Ax * By) = pressA

        // println!("all right, calculating values for claw machine\n{:?}", self);
        let press_a = (self.button_b.horizontal * self.target.vertical
            - self.button_b.vertical * self.target.horizontal) as f64
        / (self.button_a.vertical * self.button_b.horizontal - self.button_a.horizontal * self.button_b.vertical) as f64;

        // println!("press a: {}", press_a);

        if press_a.fract() != 0f64 {
            // println!("-- press a fraction is : {}", press_a.fract());
            return None
        }

        // Find pressB
        // target.x = pressA * A.x + pressB * B.x
        // target.x - pressA * A.x = pressB * B.x
        // (target.x - pressA * A.x) / B.x = pressB

        let press_b = (self.target.horizontal as f64 - press_a * self.button_a.horizontal as f64) / self.button_b.horizontal as f64;

        // println!("press b: {}", press_b);
        if press_b.fract() != 0f64 {
            // println!("-- press b fraction is : {}", press_b.fract());
            return None
        }

        Some((press_a * self.price_a as f64 + press_b * self.price_b as f64) as i64)
    }
}

pub(crate) fn str_to_nums(input: &str) -> Option<(i64, i64)> {
    let nums = input
        .trim()
        .split(", ")
        .map(|x| x[2..].parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    if nums.len() != 2 {
        return None
    }

    Some((nums[0], nums[1]))
}



pub(crate) fn solve(input: &str) -> i64 {
    let machines:Vec<ClawMachine> = input
        .trim()
        .split("\n\n")
        .map(|block| { ClawMachine::try_from(block).unwrap() }).collect();

    let mut sum = 0;

    for machine in machines {
        match machine.least_tokens() {
            None => {}
            Some(boop) => {
                sum += boop;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first_example() {
        let machine = ClawMachine::new(
            Vector::new(94, 34),
            Vector::new(22, 67),
            Coordinate::new(8400, 5400)
        );

        assert!(machine.least_tokens().is_some());
        assert_eq!(machine.least_tokens(), Some(280));
    }
}
