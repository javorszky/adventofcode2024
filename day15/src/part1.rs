use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};


pub(crate) struct BadDay15Error{
    pub(crate) msg: String
}

impl Display for BadDay15Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad input")
    }
}

impl Debug for BadDay15Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad input: {}", self.msg)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Entity {
    Box,
    Wall,
    Robot,
    Empty,
    BoxLeft,
    BoxRight
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum MoveCommand {
    Up,
    Left,
    Down,
    Right
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) struct Coordinate {
    pub(crate) horizontal: u32,
    pub(crate) vertical: u32,
}

impl Coordinate {
    pub(crate) fn try_from(horizontal: usize, vertical: usize) -> Coordinate {
        Coordinate::new(horizontal as u32,vertical as u32)
    }

    pub(crate) fn new(horizontal: u32, vertical: u32) -> Coordinate {
        Coordinate{horizontal, vertical}
    }

    pub(crate) fn next(&self, direction: MoveCommand) -> Coordinate {
        match direction {
            MoveCommand::Up => {Coordinate::new(self.horizontal, self.vertical-1)}
            MoveCommand::Left => {Coordinate::new(self.horizontal-1, self.vertical)}
            MoveCommand::Down => {Coordinate::new(self.horizontal, self.vertical+1)}
            MoveCommand::Right => {Coordinate::new(self.horizontal+1, self.vertical)}
        }
    }
}

pub(crate) struct Day15 {
    pub(crate) map: HashMap<Coordinate, Entity>,
    pub(crate) commands: Vec<MoveCommand>,
    pub(crate) robot: Coordinate,
    pub(crate) bounds: (u32, u32)
}

impl Day15 {
    fn try_from(input: &str) -> Result<Day15, BadDay15Error> {
        let parts = input.trim().split("\n\n").collect::<Vec<_>>();

        let commands = parse_commands(parts[1])?;
        let (map, robot_coordinate, bounds) = parse_map(parts[0])?;

        Ok(Day15::new(map, commands, robot_coordinate, bounds))
    }

    pub(crate) fn new(
        map: HashMap<Coordinate, Entity>,
        commands: Vec<MoveCommand>,
        robot: Coordinate,
        bounds: (u32, u32)
    ) -> Day15 {
        Day15 { map, commands, robot, bounds }
    }

    fn walk(&mut self) {
        for c in self.commands.clone() {
            self.execute_command(c);
        }
    }

    fn count_gps(&self) -> u32 {
        self.map.iter().fold(0, |acc, (c, e)| {
            let mut res = acc;
            if e == &Entity::Box {
                res = acc + 100 * c.vertical + c.horizontal
            }

            res
        })
    }


    fn execute_command(&mut self, command: MoveCommand) {
        // get where the robot is
        let mut current = self.robot;
        let mut previous = Entity::Robot;
        let mut replacements: HashMap<Coordinate, (Entity, Entity)> = HashMap::new();

        // let's add the robot to the top of the replacements for the current coordinate.
        replacements.insert(current, (Entity::Robot, Entity::Empty));

        loop {
            let next = current.next(command);

            match self.map.get(&next) {
                Some(Entity::Box) => {
                    replacements.insert(next, (Entity::Box, previous));
                    current = next;
                    previous = Entity::Box;
                }
                Some(Entity::Wall) => {
                    // nothing happens here, nothing gets replaced, nothing moves
                    replacements = HashMap::new();
                    break;
                }
                Some(Entity::Robot) => {
                    panic!("Going away from robot should never encounter a robot!");
                }
                Some(Entity::Empty) => {
                    // we found an empty spot, everything can thus move
                    replacements.insert(next, (Entity::Empty, previous));
                    break;
                }
                _ => {
                    panic!("we encountered something that absolutely should not exist")
                }
            }
        }

        if replacements.is_empty() {
            return;
        }

        for (coord, (_from_entity, to_entity)) in replacements.iter() {
            self.map.insert(*coord, *to_entity);
            if *to_entity == Entity::Robot {
                self.robot = *coord;
            }
        }
    }
}


pub(crate) fn parse_commands(input: &str) -> Result<Vec<MoveCommand>, BadDay15Error> {
    let mut commands = Vec::new();

    // Parse the commands
    for (idx, c) in input.replace("\n", "").trim().chars().enumerate() {
        let command = match c {
            '<' => MoveCommand::Left,
            '>' => MoveCommand::Right,
            '^' => MoveCommand::Up,
            'v' => MoveCommand::Down,
            _ => {
                println!("Illegal character found for char {} at idx {}", c, idx);

                return Err(BadDay15Error{msg: format!("Unexpected character '{}'", c)})}
        };

        commands.push(command);
    }

    Ok(commands)
}

pub(crate) fn parse_map(input: &str) -> Result<(HashMap<Coordinate, Entity>, Coordinate, (u32, u32)), BadDay15Error> {
    let mut map: HashMap<Coordinate, Entity> = HashMap::new();
    let mut robot_coordinate: Option<Coordinate> = None;
    let mut max_h = 0;
    let mut max_v = 0;

    for (vertical, l) in input.trim().lines().enumerate() {
        if vertical > max_v {
            max_v = vertical;
        }

        for (horizontal, c) in l.chars().enumerate() {
            if horizontal > max_h {
                max_h = horizontal;
            }

            let e = match c {
                '#' => Entity::Wall,
                '.' => Entity::Empty,
                'O' => Entity::Box,
                '@' => {
                    robot_coordinate = Some(Coordinate::try_from(horizontal, vertical));
                    Entity::Robot
                },
                _ => {
                    println!("Illegal character found at line {} char {}", vertical, horizontal
                    );
                    return Err(BadDay15Error{msg: format!("Unexpected character '{}'", c)})}
            };

            // insert the first one as normal
            map.insert(Coordinate::try_from(horizontal, vertical), e);
        }
    }

    if robot_coordinate.is_none() {
        return Err(BadDay15Error{msg: "No robot found".to_string()})
    }

    Ok((map, robot_coordinate.unwrap(), (max_h as u32, max_v as u32)))
}


pub(crate) fn solve(input: &str) -> u32 {
    let mut d = Day15::try_from(input).unwrap();
    d.walk();

    d.count_gps()
}