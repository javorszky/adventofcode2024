use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
struct BadRobotError {
    msg: String,
}

impl BadRobotError {
    fn new(msg: &str) -> BadRobotError {
        BadRobotError { msg: msg.to_string() }
    }
}

impl Debug for BadRobotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not instantiate a robot from input '{}'", self.msg)
    }
}

impl Display for BadRobotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not instantiate a robot")
    }
}

#[derive(Debug)]
struct Robot {
    starting_position_horizontal: i32,
    starting_position_vertical: i32,
    v_horizontal: i32,
    v_vertical: i32,
}


impl Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Robot{{ x({}, {}), ->({}, {})}}",
            self.starting_position_horizontal,
            self.starting_position_vertical,
            self.v_horizontal,
            self.v_vertical
        )
    }
}

impl Robot {
    fn try_from(input: &str) -> Result<Robot, BadRobotError> {
        // p=0,0 v=1,3
        let parts: Vec<Vec<i32>> = input.split_whitespace().map(|line| {
            line[2..].split(',').map(|part| part.parse().unwrap()).collect::<Vec<i32>>()})
            .collect();

        if parts.len() != 2 || parts[0].len() != 2 || parts[1].len() != 2 {
            return Err(BadRobotError::new(input));
        }

        Ok(Robot::new(parts[0][0], parts[0][1],parts[1][0], parts[1][1]))
    }

    fn new(pos_h: i32, pos_v: i32, vel_h: i32, vel_v: i32) -> Robot {
        Robot {
            starting_position_horizontal: pos_h,
            starting_position_vertical: pos_v,
            v_horizontal: vel_h,
            v_vertical: vel_v,
        }
    }

    /// Method gets passed a tuple of numbers: the width and height of the field we're looking at,
    /// and the number of times it needs to move. The result is the coordinate it ends up after
    /// <times> number of hops.
    fn move_robot(&self, bounds: (i32, i32), times: i32) -> (i32, i32) {
        // println!("\nMoving robot {} {} times within bounds {:?}", self, times, bounds);
        let mut res_h = (self.starting_position_horizontal + times * self.v_horizontal) % (bounds.0);
        let mut res_v = (self.starting_position_vertical + times * self.v_vertical) % (bounds.1);

        // println!("starting {} + v horizontal {} * {} times is {}, mod {} is {}",
        //          self.starting_position_horizontal, self.v_horizontal, times,
        //          self.starting_position_horizontal + times * self.v_horizontal, bounds.0, res_h
        // );
        //
        // println!("starting {} + v vertical {} * {} times is {}, mod {} is {}",
        //          self.starting_position_vertical, self.v_vertical, times,
        //          self.starting_position_vertical + times * self.v_vertical,
        //          bounds.1, res_v);

        if res_h.cmp(&0) == Ordering::Less { res_h += bounds.0 }

        if res_v.cmp(&0) == Ordering::Less { res_v += bounds.1 }

        // println!("after wraparound checks, resh and resv are {}, {}", res_h, res_v);



        (res_h, res_v)
    }
}

struct Day14 {
    width: i32,
    height: i32,
    steps: i32,
    robots: Vec<Robot>,
}

impl Day14 {
    fn new(width: i32, height: i32, steps: i32, robots: Vec<Robot>) -> Day14 {
        Day14{width, height, steps, robots}
    }

    fn safety_score(&self) -> i32 {
        let omit_h = (self.width - 1) / 2;
        let omit_v = (self.height - 1) / 2;

        // println!("Omit h: {}, omit v: {} for width {} height {}", omit_h, omit_v, self.width, self.height);

        let robot_positions = self.robots.iter()
            .map(|robot| {
                robot.move_robot((self.width, self.height), self.steps)
            })
            .collect::<Vec<(i32, i32)>>();

        // println!("{:?}", &robot_positions);

        let omitted_robot_positions = robot_positions.into_iter()
            .filter(|&position| {
                position.0 != omit_h && position.1 != omit_v
            }).collect::<Vec<(i32, i32)>>();

        let mut safety_score = 1; // because we're multiplying

        for q in [Quadrant::TopLeft,Quadrant::TopRight,Quadrant::BottomLeft,Quadrant::BottomRight].iter() {
            let q_bounds:((i32, i32), (i32, i32)) = match q {
                Quadrant::TopLeft => { ((0,0), (omit_h, omit_v))}
                Quadrant::TopRight => { ((omit_h, 0), (self.width, omit_h)) }
                Quadrant::BottomLeft => { ((0, omit_v), (omit_h, self.height))}
                Quadrant::BottomRight => { ((omit_h, omit_v), (self.width, self.height))}
            };

            let robots_in_quadrant = omitted_robot_positions.iter()
                .filter(|&position| {
                    if position.0 < q_bounds.0.0 || position.0 > q_bounds.1.0 { return false }
                    if position.1 < q_bounds.0.1 || position.1 > q_bounds.1.1 { return false }
                    true
                }).count() as i32;

            // println!("robots in quadrant {:?}: {}", q, robots_in_quadrant);

            safety_score *= robots_in_quadrant;
        }


        safety_score
    }
}

pub(crate) fn solve(input: &str, width: i32, height: i32, steps: i32) -> i32 {
    let robots: Vec<Robot> = input.trim().lines().map(|line| Robot::try_from(line).unwrap()).collect();
    let d14 = Day14::new(width, height, steps, robots);

    d14.safety_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_move() {
        // ...
        // .r.
        // ...
        //
        // p=1,1 v=1,-1


        let r = Robot::try_from("p=1,1 v=1,-1");
        assert!(r.is_ok());

        let bounds = (3,3);

        let robot = r.unwrap();

        let (mut horizontal, mut vertical) = robot.move_robot(bounds, 1);
        assert_eq!((horizontal, vertical), (2, 0));

        (horizontal, vertical) = robot.move_robot(bounds, 2);
        assert_eq!((horizontal, vertical), (0, 2));

        (horizontal, vertical) = robot.move_robot(bounds, 3);
        assert_eq!((horizontal, vertical), (1, 1));

        (horizontal, vertical) = robot.move_robot(bounds, 27);
        assert_eq!((horizontal, vertical), (1, 1));
    }
}