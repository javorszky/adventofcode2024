use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};


struct BadDay15Error{
    msg: String
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

#[derive(Copy, Clone, Eq, PartialEq)]
enum Entity {
    Box,
    Wall,
    Robot,
    Empty
}

#[derive(Copy, Clone)]
enum MoveCommand {
    Up,
    Left,
    Down,
    Right
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coordinate {
    horizontal: u32,
    vertical: u32,
}

impl Coordinate {
    fn try_from(horizontal: usize, vertical: usize) -> Coordinate {
        Coordinate::new(horizontal as u32,vertical as u32)
    }

    fn new(horizontal: u32, vertical: u32) -> Coordinate {
        Coordinate{horizontal, vertical}
    }

    fn next(&self, direction: MoveCommand) -> Coordinate {
        match direction {
            MoveCommand::Up => {Coordinate::new(self.horizontal, self.vertical-1)}
            MoveCommand::Left => {Coordinate::new(self.horizontal-1, self.vertical)}
            MoveCommand::Down => {Coordinate::new(self.horizontal, self.vertical+1)}
            MoveCommand::Right => {Coordinate::new(self.horizontal+1, self.vertical)}
        }
    }
}

struct Day15 {
    map: HashMap<Coordinate, Entity>,
    commands: Vec<MoveCommand>,
    robot: Coordinate,
}


impl Day15 {
    fn try_from(input: &str) -> Result<Day15, BadDay15Error> {
        let parts = input.trim().split("\n\n").collect::<Vec<_>>();

        // Parse the map
        let mut map: HashMap<Coordinate, Entity> = HashMap::new();
        let mut robot_coordinate: Option<Coordinate> = None;

        let map_input = parts[0].trim();

        for (vertical, l) in map_input.lines().enumerate() {
            for (horizontal, c) in l.chars().enumerate() {
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

                map.insert(Coordinate::try_from(horizontal, vertical), e);
            }
        }

        if robot_coordinate.is_none() {
            return Err(BadDay15Error{msg: "No robot found".to_string()})
        }

        let mut commands = Vec::new();

        // Parse the commands
        for (idx, c) in parts[1].replace("\n", "").trim().chars().enumerate() {
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

        Ok(Day15{map, commands, robot: robot_coordinate.unwrap()})
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

pub(crate) fn solve(input: &str) -> u32 {
    let mut d = Day15::try_from(input).unwrap();
    d.walk();

    d.count_gps()
}