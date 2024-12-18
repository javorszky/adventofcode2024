use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::vec;
use crate::part1::{parse_commands, parse_map, BadDay15Error, Coordinate, Day15, Entity, MoveCommand};


struct Replacement {
    depth: u32,
    coordinate: Coordinate,
    entity: Entity
}

impl Replacement {
    fn new(depth: u32, coordinate: Coordinate, entity: Entity) -> Self {
        Self { depth, coordinate, entity }
    }
}


impl Day15 {
    fn try_from_part2(input: &str) -> Result<Day15, BadDay15Error> {
        let parts = input.trim().split("\n\n").collect::<Vec<_>>();

        let (standard_map, _robot_coordinate, _bad_bounds) = parse_map(parts[0])?;
        let (wide_map, (widest, tallest), robot) = widen_map(standard_map);




        // println!("{}",draw_map(wide_map.clone(), (widest, tallest)));

        let commands = parse_commands(parts[1])?;
        // println!("commands in order at start:\n{:?}", commands);

        Ok(Day15::new(wide_map, commands, robot, (widest, tallest)))
    }

    fn walk_part2(&mut self) {
        for (i, c) in self.commands.clone().iter().enumerate() {
            // println!("\
            // ========================\n\
            // Iteration {:3}: {:?}\n\
            // ========================", i, c);
            // println!("all right, let's start with robot at {:?} {{", self.robot);
            if let Some(mut replacements) = self.push(Entity::Empty, Entity::Robot, self.robot, *c, 0) {
                // println!("}} got the following replacements at the end of robots:\n{:?}", replacements);

                // sort replacements by their coordinate
                replacements.sort_by(|a, b| a.depth.cmp(&b.depth));

                let mut replacements_done: HashSet<Coordinate> = HashSet::new();

                for r in replacements.into_iter() {
                    if r.entity == Entity::Robot {
                        self.robot = r.coordinate;
                    }

                    if replacements_done.contains(&r.coordinate) {
                        continue;
                    }

                    self.map.insert(r.coordinate, r.entity);

                    if r.entity != Entity::Empty {
                        replacements_done.insert(r.coordinate);
                    }
                }
            } else {
                // println!("nah, apparently this was a None");
            }
            //
            // println!("map after moving {:?}", c);
            // println!("{}", draw_map(self.map.clone(), self.bounds));
            // println!("\n");
        }
    }


    /// Pass in the previous entity, and the current coordinate. The previous entity is usually what
    /// is on the current coordinate at the current time, so when we move one tile in the direction,
    /// what the current one is going to be replaced with.
    fn push(&self, replacement: Entity, current: Entity, what: Coordinate, which_way: MoveCommand, depth: u32) -> Option<Vec<Replacement>> {
        let _prefix = "  ".repeat(depth as usize);
        // println!("{}d{}: moving {:?} {:?} from {:?}, replacing it with {:?}", prefix, depth, current, which_way, what, replacement);
        // what is the current coordinate, so we need to get the next one in the which way
        let next = what.next(which_way);
        let mut replacements: Vec<Replacement> = vec![Replacement::new(depth, what, replacement)];
        // println!("{}d{}: replacements to start is {:?}", prefix, depth, replacements);
        // println!("{}d{}: checking the next coordinate {:?}", prefix, depth, next);

        match self.map.get(&next).unwrap() {
            Entity::Box => { panic!("we're in part 2, we should only have box left or box right..."); }
            Entity::Wall => {
                // println!("{}d{}: next tile was a {:?}, returning None", prefix, depth, Entity::Wall);
                None
            }
            Entity::Robot => { panic!("we should not encounter another robot...") }
            Entity::Empty => {
                // println!("{}d{}: encountered an empty, adding ({:?}, {:?}) to the replacements", prefix, depth, next, current);

                replacements.push(Replacement::new(depth, next, current));
                // println!("{}d{}: returning the replacements so far: {:?}", prefix, depth, replacements);
                Some(replacements)
            }
            Entity::BoxLeft => {
                // println!("{}d{}: encountered a BoxLeft [", prefix, depth);
                match which_way {
                    MoveCommand::Up|MoveCommand::Down => {
                        // println!("{}d{}: moving towards {:?}", prefix, depth, which_way);
                        // println!("{}d{}: calling push for {:?} to be replaced by {:?}", prefix, depth, Entity::BoxLeft, current);

                        match self.push(current, Entity::BoxLeft, next, which_way, depth + 1) {
                            Some(mut res_left) => {

                                // println!("res left is {:?}", res_left);
                                // println!("{}d{}: got a Some {:?}", prefix, depth, res_right);
                                // for (coordinate, new_entity) in res_left.clone().drain() {
                                //     if replacements.contains_key(&coordinate) && new_entity == Entity::Empty {
                                //         // println!("replacements already contain something on coordinate {:?}", coordinate);
                                //         // println!("existing value is {:?}, incoming value is {:?}", replacements.get(&coordinate).unwrap(), new_entity);
                                //         continue;
                                //     }
                                //
                                //     replacements.insert(coordinate, new_entity);
                                //     // if new_entity == Entity::Empty {
                                //     //
                                //     // }
                                // }
                                // println!("{}d{}: got a Some {:?}", prefix, depth, res_left);
                                // replacements.extend(res_left);
                                replacements.append(&mut res_left);
                                // println!("{}d{}: resulting in a new replacement of {:?}", prefix, depth, replacements);
                            }
                            None => {
                                // println!("{}d{}: pushing box {:?} found a None, returning None", prefix, depth, which_way);
                                return None
                            }
                        }

                        if current == Entity::BoxLeft {
                            // box left is pushing up box left, which means we already accounted for
                            // the other side
                            // println!("{}d{}: a {:?} is pushing a BoxLeft, returning current replacements: {:?}", prefix, depth, current, replacements);
                            return Some(replacements);
                        }

                        // println!("{}d{}: all right, pushing box {:?} was a Some, lets check the other side!", prefix, depth, which_way);
                        let box_right_coordinate = next.next(MoveCommand::Right);

                        // println!("{}d{}: original coordinate was {:?}, new one is {:?}", prefix, depth,
                        // what, box_right_coordinate);

                        match self.push(Entity::Empty, Entity::BoxRight, box_right_coordinate, which_way, depth + 1) {
                            Some(mut res_right) => {
                                // println!("res right is {:?}", res_right);
                                // println!("{}d{}: got a Some {:?}", prefix, depth, res_right);
                                // for (coordinate, new_entity) in res_right.clone().drain() {
                                //     if replacements.contains_key(&coordinate) && new_entity == Entity::Empty {
                                //         // println!("replacements already contain something on coordinate {:?}", coordinate);
                                //         // println!("existing value is {:?}, incoming value is {:?}", replacements.get(&coordinate).unwrap(), new_entity);
                                //         continue;
                                //     }
                                //
                                //     replacements.insert(coordinate, new_entity);
                                //     // if new_entity == Entity::Empty {
                                //     //
                                //     // }
                                // }

                                replacements.append(&mut res_right);
                                // println!("{}d{}: resulting in a new replacement of {:?}", prefix, depth, replacements);
                            }
                            None => {
                                // println!("{}d{}: pushing box found a None, returning None", prefix, depth);
                                return None }
                        }

                        // println!("{}d{} both box left and right appended to the replacements, resulting in the returning of\n\
                        // {:?}", prefix, depth, replacements);

                        Some(replacements)
                    }
                    MoveCommand::Left => {
                        // means we're coming from the right, box left should never be encountered
                        // this way!
                        panic!("we're coming from the right moving left, encountering a [ should \
                        not have happened!")
                    }
                    MoveCommand::Right => {
                        // we're encountering a BoxLeft moving Right
                        // ->[
                        // which means we can immediately add this into the replacements
                        // means we're coming from the left, so encountering box left [ is expected
                        // println!("{}d{}: we've encountered a box left while going right", prefix, depth);
                        replacements.push(Replacement::new(depth, next, current));
                        // println!("{}d{}: added BoxLeft and this coordinate to the replacements", prefix, depth);

                        let skip = next.next(which_way); // skip one
                        // println!("{}d{}: skipping to find the ] at coordinate {:?}", prefix, depth, skip);
                        match self.push(Entity::BoxLeft, Entity::BoxRight, skip, which_way, depth + 1) {
                            Some(mut res) => {
                                // println!("{}d{}: got some results ({:?}) from pushing box right to the right", prefix, depth, res);
                                replacements.append(&mut res);
                                // println!("{}d{}: and appended the results to get the replacements of\n{:?}", prefix, depth, replacements);
                                Some(replacements)
                            }
                            None => {
                                // println!("{}d{}: pushing box right to the right encountered a None, returning None", prefix, depth);
                                None
                            }
                        }
                    }
                }
            }
            Entity::BoxRight => {
                // println!("{}d{}: encountered a BoxRight", prefix, depth);
                match which_way {
                    MoveCommand::Up|MoveCommand::Down => {
                        // println!("{}d{}: moving towards {:?}", prefix, depth, which_way);
                        // println!("{}d{}: calling push for {:?} to be replaced by {:?}", prefix, depth, Entity::BoxRight, current);

                        // means we're going from the bottom, or top, which means we also need to
                        // check the one to the right.
                        match self.push(current, Entity::BoxRight, next, which_way, depth + 1) {
                        // match self.push(current, Entity::BoxRight, next, which_way, depth + 1) {
                            Some(mut res_right) => {
                                // println!("{}d{}: got a Some {:?}", prefix, depth, res_right);
                                replacements.append(&mut res_right);
                                // println!("{}d{}: resulting in a new replacement of {:?}", prefix, depth, replacements);
                            }
                            None => {
                                // println!("{}d{}: pushing box {:?} found a None, returning None", prefix, depth, which_way);
                                return None }
                        }
                        // println!("{}d{}: all right, pushing BoxRight was a Some, lets check the other side!", prefix, depth);

                        if current == Entity::BoxRight {
                            // box right is pushing up box right, which means the left side has
                            // already been accounted for previously
                            // println!("{}d{}: a {:?} is pushing a BoxRight, returning current replacements: {:?}", prefix, depth, current, replacements);

                            return Some(replacements);
                        }

                        let box_left_coordinate = next.next(MoveCommand::Left);
                        // println!("{}d{}: original coordinate was {:?}, new one is (to the left and vertical dir +1) {:?}", prefix, depth,
                        //          what, box_left_coordinate);

                        // println!("{}d{}: on coordinate {:?} there is a BoxLeft, which we're replacing with an Empty", prefix, depth, box_left_coordinate);

                        match self.push(Entity::Empty, Entity::BoxLeft, box_left_coordinate, which_way, depth + 1) {
                            None => {
                                // println!("{}d{}: pushing box left found a None, returning None", prefix, depth);
                                return None;
                            }
                            Some(mut res_right) => {
                                // println!("{}d{}: we pushed {:?} and got a result of {:?}",prefix,  depth, which_way, res_right);
                                replacements.append(&mut res_right);
                                // println!("{}d{}: resulting in a new replacement of {:?}", prefix, depth, replacements);
                            }
                        }

                        // println!("{}d{} both box left and right appended to the replacements, resulting in the returning of\n\
                        // {:?}",prefix, depth, replacements);

                        Some(replacements)
                    }
                    MoveCommand::Left => {
                        // we have encountered a box rith ] while moving left
                        // []<-
                        // means we're coming from the left, so encountering box right ] is expected
                        // println!("{}d{}: we're going to add this BoxRight at {:?} to the replacements", prefix,  depth, next);
                        replacements.push(Replacement::new(depth, next, current));

                        let skip = next.next(which_way); // skip one
                        // println!("{}d{}: skipping to find the [ at coordinate {:?}", prefix, depth, skip);

                        match self.push(Entity::BoxRight, Entity::BoxLeft, skip, which_way, depth + 1) {
                            Some(mut res) => {
                                // println!("{}d{}: got some results ({:?}) from pushing box right to the right",prefix,  depth, res);

                                replacements.append(&mut res);
                                // println!("{}d{}: and appended the results to get the replacements of\n{:?}", prefix, depth, replacements);
                                Some(replacements)
                            }
                            None => {
                                // println!("{}d{}: pushing box left to the left encountered a None, returning None",prefix,  depth);

                                None }
                        }

                    }
                    MoveCommand::Right => {
                        // means we're coming from the right, box left should never be encountered
                        // this way!
                        panic!("we're coming from the left moving right, encountering a ] should \
                        not have happened!")
                    }
                }
            }
        }
    }

    fn count_gps_part2(&self) -> u32 {
        self.map.iter()
            .filter(|&(_k, v)| v == &Entity::BoxLeft)
            .collect::<HashMap<_, _>>()
            .iter()
            .fold(0, |acc, (&k, &_v)| {
                acc + k.horizontal + 100 * k.vertical
        })
    }



    // fn push_better(&self, what: Entity, coordinate: Coordinate, which_way: MoveCommand, what_pushed_this: Entity) -> Option<HashMap<Coordinate, Entity>> {
    //     // This entity is being pushed
    //     match what {
    //         Entity::Box => {
    //             panic!("in part 2 there's no box, just box left or box right");
    //         }
    //         Entity::Wall => {
    //             // I am a wall being pushed, I do not want to be pushed, everyone behind me can suck it
    //             return None
    //         }
    //         Entity::Robot => {
    //             panic!("there is only one robot, and it's the opposite direction");
    //         }
    //         Entity::Empty => {
    //             // this is an empty space, let's replace this with whatever is needed
    //             return Some(HashMap::<Coordinate, Entity>::from([(coordinate, what_pushed_this)]));
    //         }
    //         Entity::BoxLeft => {
    //             // I am a box left, which means I need to also push box right
    //
    //         }
    //         Entity::BoxRight => {}
    //     }
    //
    //     // next coordinate is origin + which way
    //     let next_coordinate = origin.next(which_way);
    //     let entity_being_pushed = self.map.get(&next_coordinate).unwrap();
    //
    //     match entity_being_pushed {
    //         Entity::Box => {}
    //         Entity::Wall => {
    //             // pushing a wall is a no-go, so we're returning a None
    //             return None;
    //         }
    //         Entity::Robot => {
    //             panic!("we should not encounter another robot...")
    //         }
    //         Entity::Empty => {}
    //         Entity::BoxLeft => {}
    //         Entity::BoxRight => {}
    //     }
    //
    //
    //     None
    // }
}

fn widen_map(source: HashMap<Coordinate, Entity>) -> (HashMap<Coordinate, Entity>, (u32, u32), Coordinate) {
    let mut widest = 0;
    let mut tallest = 0;

    let mut robot = Coordinate::new(0, 0);
    let mut wide_map: HashMap<Coordinate, Entity> = HashMap::new();
    for (c, e) in source.clone().into_iter() {
        if c.horizontal > widest { widest = c.horizontal }
        if c.vertical > tallest { tallest = c.vertical }

        match e {
            Entity::Robot => {
                // '@' -> '@.'
                wide_map.insert(Coordinate::new(c.horizontal * 2, c.vertical), e);
                robot = Coordinate::new(c.horizontal * 2, c.vertical);
                wide_map.insert(Coordinate::new(c.horizontal * 2 + 1, c.vertical), Entity::Empty);
            }
            Entity::Box => {
                // 'O' -> '[]'
                wide_map.insert(Coordinate::new(c.horizontal * 2, c.vertical), Entity::BoxLeft);
                wide_map.insert(Coordinate::new(c.horizontal * 2 + 1, c.vertical), Entity::BoxRight);
            }
            _ => {
                // '.' -> '..'
                // '#' -> '##'
                wide_map.insert(Coordinate::new(c.horizontal * 2, c.vertical), e);
                wide_map.insert(Coordinate::new(c.horizontal * 2 + 1, c.vertical), e);
            }
        }
    }

    (wide_map, (widest*2+1, tallest), robot)
}

fn draw_map(map: HashMap<Coordinate, Entity>, bounds: (u32, u32)) -> String {
    let mut s = "".to_string();

    for vertical in 0..=bounds.1 {
        for horizontal in 0..=bounds.0 {
            let tile = match map.get(&Coordinate::new(horizontal, vertical)).unwrap() {
                Entity::Box => { "O" }
                Entity::Wall => { "#" }
                Entity::Robot => { "@" }
                Entity::Empty => { "." }
                Entity::BoxLeft => { "[" }
                Entity::BoxRight => { "]" }
            };

            s += tile
        }

        s += "\n";
    }

    s
}


/// Returns a sort function that will put the vecs in application order depending on which way
/// the push was going. For off-axis ordering it's always top to bottom or left to right given
/// same in-axis values.
fn sort_function(direction: MoveCommand) -> fn(&Replacement, &Replacement) -> Ordering {
    match direction {
        MoveCommand::Up => {
            |a: &Replacement, b: &Replacement| -> Ordering {
                // we're going up, so the coordinate with the lower vertical wins
                if a.coordinate.vertical == b.coordinate.vertical {
                    return a.coordinate.horizontal.cmp(&b.coordinate.horizontal);
                }

                a.coordinate.vertical.cmp(&b.coordinate.vertical)
            }
        }
        MoveCommand::Left => {
            |a: &Replacement, b: &Replacement| -> Ordering {
                // we're going left, so the coordinate with the lower horizontal wins
                if a.coordinate.horizontal == b.coordinate.horizontal {
                    return a.coordinate.vertical.cmp(&b.coordinate.vertical);
                }

                a.coordinate.horizontal.cmp(&b.coordinate.horizontal)
            }
        }
        MoveCommand::Down => {
            |a: &Replacement, b: &Replacement| -> Ordering {

                // we're going down, so the coordinate with the higher vertical wins
                if a.coordinate.vertical == b.coordinate.vertical {
                    return a.coordinate.horizontal.cmp(&b.coordinate.horizontal);
                }

                b.coordinate.vertical.cmp(&a.coordinate.vertical)
            }
        }
        MoveCommand::Right => {
            |a: &Replacement, b: &Replacement| -> Ordering {
                // we're going right, so the coordinate with the higher horizontal wins
                if a.coordinate.horizontal == b.coordinate.horizontal {
                    return a.coordinate.vertical.cmp(&b.coordinate.vertical);
                }

                b.coordinate.horizontal.cmp(&a.coordinate.horizontal)
            }
        }
    }
}


pub(crate) fn solve(input: &str) -> u32 {
    let mut d = Day15::try_from_part2(input).unwrap();
    // println!("created d");

    d.walk_part2();
    // println!("did the walk part 2 thing");

    // println!("okay, so counting gps");

    d.count_gps_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_block_right() {
        let input = "\
        #####\n\
        #@O.#\n\
        #####\n\
        \n\
        >>\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ##########\n\
        ##..@[].##\n\
        ##########\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_left() {
        let input = "\
        #####\n\
        #.O@#\n\
        #####\n\
        \n\
        <\n"; // only one left is needed because the @ will be on the left side after expanding

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ##########\n\
        ##.[]@..##\n\
        ##########\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_up() {
        let input = "\
        #####\n\
        #...#\n\
        #.O.#\n\
        #.@.#\n\
        #####\n\
        \n\
        ^\n"; // only one left is needed because the @ will be on the left side after expanding

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ##########\n\
        ##..[]..##\n\
        ##..@...##\n\
        ##......##\n\
        ##########\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_down() {
        let input = "\
        #####\n\
        #.@.#\n\
        #.O.#\n\
        #...#\n\
        #####\n\
        \n\
        v\n"; // only one left is needed because the @ will be on the left side after expanding

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ##########\n\
        ##......##\n\
        ##..@...##\n\
        ##..[]..##\n\
        ##########\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_another_box_left() {
        let input = "\
        ######\n\
        #....#\n\
        #.OO@#\n\
        #....#\n\
        ######\n\
        \n\
        <\n"; // only one left is needed because the @ will be on the left side after expanding

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##........##\n\
        ##.[][]@..##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_another_box_right() {
        let input = "\
        ######\n\
        #....#\n\
        #@OO.#\n\
        #....#\n\
        ######\n\
        \n\
        >>\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##........##\n\
        ##..@[][].##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_another_box_up() {
        let input = "\
        ######\n\
        #....#\n\
        #.OO.#\n\
        #..O@#\n\
        #....#\n\
        ######\n\
        \n\
        <v<^\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##..[][]..##\n\
        ##...[]...##\n\
        ##....@...##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_other_boxes_down() {
        let input = "\
        ######\n\
        #..@.#\n\
        #..O.#\n\
        #..O.#\n\
        #..O.#\n\
        #..O.#\n\
        #.OO.#\n\
        #....#\n\
        #....#\n\
        ######\n\
        \n\
        >>vvvv<>^^<>^^<v\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##........##\n\
        ##.....@..##\n\
        ##....[]..##\n\
        ##...[]...##\n\
        ##....[]..##\n\
        ##...[]...##\n\
        ##..[][]..##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_other_boxes_down_from_left() {
        let input = "\
        ######\n\
        #.@..#\n\
        #.O..#\n\
        #.O..#\n\
        #.O..#\n\
        #.O..#\n\
        #.OO.#\n\
        #....#\n\
        #....#\n\
        ######\n\
        \n\
        <vvvv><^^><^^>v\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##........##\n\
        ##..@.....##\n\
        ##..[]....##\n\
        ##...[]...##\n\
        ##..[]....##\n\
        ##...[]...##\n\
        ##..[][]..##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }


    #[test]
    fn test_moving_block_push_two_levels_other_boxes_down() {
        let input = "\
        #######\n\
        #..@..#\n\
        #..O..#\n\
        #.OO..#\n\
        #.OOO.#\n\
        #.....#\n\
        #######\n\
        \n\
        <<<vv>^^>>v\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ##############\n\
        ##..........##\n\
        ##....@.....##\n\
        ##....[]....##\n\
        ##...[][]...##\n\
        ##..[][][]..##\n\
        ##############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_another_box_up_straight() {
        let input = "\
        ######\n\
        #....#\n\
        #..O.#\n\
        #..O.#\n\
        #..@.#\n\
        ######\n\
        \n\
        ^\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##....[]..##\n\
        ##....[]..##\n\
        ##....@...##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_up_straight() {
        let input = "\
        ######\n\
        #....#\n\
        #....#\n\
        #..O.#\n\
        #..@.#\n\
        ######\n\
        \n\
        ^\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##........##\n\
        ##....[]..##\n\
        ##....@...##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_another_box_up_straight_on_the_right() {
        let input = "\
        ######\n\
        #....#\n\
        #..O.#\n\
        #..O.#\n\
        #..@.#\n\
        ######\n\
        \n\
        >^\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##....[]..##\n\
        ##....[]..##\n\
        ##.....@..##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_another_box_up_jagged() {
        let input = "\
        ######\n\
        #....#\n\
        #..O.#\n\
        #..O@#\n\
        #..O.#\n\
        #....#\n\
        ######\n\
        \n\
        <>vv<^\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##....[]..##\n\
        ##...[]...##\n\
        ##....[]..##\n\
        ##.....@..##\n\
        ##........##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_another_box_down_jagged() {
        let input = "\
        ######\n\
        #....#\n\
        #..O.#\n\
        #..O@#\n\
        #..O.#\n\
        #....#\n\
        ######\n\
        \n\
        <>^^<v\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ############\n\
        ##........##\n\
        ##.....@..##\n\
        ##....[]..##\n\
        ##...[]...##\n\
        ##....[]..##\n\
        ############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    #[test]
    fn test_moving_block_push_against_wall() {
        let input = "\
        #######\n\
        #.....#\n\
        #..O..#\n\
        #.O#O.#\n\
        #..O..#\n\
        #..@..#\n\
        #######\n\
        \n\
        ^>>>>^^<^^<<<<v<<<vv>vv\n";

        let mut d = Day15::try_from_part2(input).unwrap();
        d.walk_part2();
        d.count_gps_part2();

        let want_map = "\
        ##############\n\
        ##..........##\n\
        ##....[]....##\n\
        ##..[]##[]..##\n\
        ##....[]....##\n\
        ##.@........##\n\
        ##############\n";

        assert_eq!(want_map, draw_map(d.map, d.bounds));
    }

    // #[test]
    // fn test_sorter_up(){
    //     let up_sorter = sort_function(MoveCommand::Up);
    //
    //     let mut input: Vec<Replacement> = vec![
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(5,1), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(0,3), Entity::Empty),
    //     ];
    //
    //     input.sort_by(up_sorter);
    //
    //     let want_up = vec![
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(5,1), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(0,3), Entity::Empty),
    //     ];
    //
    //     assert_eq!(input.len(), want_up.len());
    //
    //     for (i, el) in input.iter().enumerate() {
    //         assert_eq!(el, &want_up[i], "index {}: sorted el: {:?}, want: {:?}", i, el, want_up[i]);
    //     }
    // }
    //
    // #[test]
    // fn test_sorter_down(){
    //     let down_sorter = sort_function(MoveCommand::Down);
    //
    //     let mut input: Vec<(Coordinate, Entity)> = vec![
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(5,1), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(0,3), Entity::Empty),
    //     ];
    //
    //     input.sort_by(down_sorter);
    //
    //     let want_down = vec![
    //         (Coordinate::new(0,3), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(5,1), Entity::Empty),
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //     ];
    //
    //     assert_eq!(input.len(), want_down.len());
    //
    //     for (i, el) in input.iter().enumerate() {
    //         assert_eq!(el, &want_down[i], "index {}: sorted el: {:?}, want: {:?}", i, el, want_down[i]);
    //     }
    // }
    //
    // #[test]
    // fn test_sorter_left(){
    //     let left_sorter = sort_function(MoveCommand::Left);
    //
    //     let mut input: Vec<(Coordinate, Entity)> = vec![
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(5,1), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(0,3), Entity::Empty),
    //     ];
    //
    //     input.sort_by(left_sorter);
    //
    //     let want_left = vec![
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(0,3), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(5,1), Entity::Empty),
    //     ];
    //
    //     assert_eq!(input.len(), want_left.len());
    //
    //     for (i, el) in input.iter().enumerate() {
    //         assert_eq!(el, &want_left[i], "index {}: sorted el: {:?}, want: {:?}", i, el, want_left[i]);
    //     }
    // }
    //
    // #[test]
    // fn test_sorter_right(){
    //     let right_sorter = sort_function(MoveCommand::Right);
    //
    //     let mut input: Vec<(Coordinate, Entity)> = vec![
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(5,1), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(0,3), Entity::Empty),
    //     ];
    //
    //     input.sort_by(right_sorter);
    //
    //     let want_right = vec![
    //         (Coordinate::new(5,1), Entity::Empty),
    //         (Coordinate::new(3,2), Entity::Empty),
    //         (Coordinate::new(2,2), Entity::Empty),
    //         (Coordinate::new(1,0), Entity::Empty),
    //         (Coordinate::new(0,0), Entity::Empty),
    //         (Coordinate::new(0,1), Entity::Empty),
    //         (Coordinate::new(0,3), Entity::Empty),
    //     ];
    //
    //     assert_eq!(input.len(), want_right.len());
    //
    //     for (i, el) in input.iter().enumerate() {
    //         assert_eq!(el, &want_right[i], "index {}: sorted el: {:?}, want: {:?}", i, el, want_right[i]);
    //     }
    // }
}