use std::collections::HashMap;

/// Coordinate is always <-> (width) first, and then height (up-down) next.
type Coordinate = (i32, i32);

const CHECKMAS: &str = "MAS";
const BAD_CHAR: &char = &'.';
const MS: &str = "MS";
const SM: &str = "SM";


pub struct Day04 {
    word_grid: HashMap<Coordinate, char>,
    x_list: Vec<Coordinate>,
    a_list: Vec<Coordinate>,
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub fn part1() -> String{
    let contents = include_str!("../input.txt");
    let d4 = Day04::new(contents);

    format!("The word XMAS can be found {} times.", d4.how_many_xmas())
}

pub fn part2() -> String{
    let contents = include_str!("../input.txt");
    let d4 = Day04::new(contents);

    format!("The word XMAS can be found {} times.", d4.how_many_x_mas_part2())
}

impl Day04 {
    pub fn new(input: &str) -> Day04 {
        let lines = input.trim().lines().collect::<Vec<&str>>();

        let mut word_grid = HashMap::<(i32, i32), char>::new();
        let mut x_list = Vec::<Coordinate>::new();
        let mut a_list = Vec::<Coordinate>::new();

        for (height, line) in lines.iter().enumerate() {
            for (width, character) in line.chars().enumerate() {
                word_grid.insert((width as i32, height as i32), character);
                if character == 'X' {
                    x_list.push((width as i32, height as i32));
                }

                if character == 'A' {
                    a_list.push((width as i32, height as i32));
                }
            }
        }

        Day04 {
            word_grid,
            x_list,
            a_list,
        }
    }

    pub fn how_many_xmas(&self) -> i32 {
        let all_directions = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest
        ];

        let mut xmases = 0;

        // let's iterate through the Xes
        for coordinate in self.x_list.iter() {
            for direction in &all_directions {
                if self.is_this_xmas(generate_coordinates_part1(coordinate, direction)) {
                    xmases += 1;
                }
            }
        }

        xmases
    }

    pub fn how_many_x_mas_part2(&self) -> i32 {
        let mut x_mas = 0;
        // for all A character
        for a in self.a_list.iter() {
            let coords = generate_coordinate_pairs_part2(a);
            if self.is_this_x_mas(&coords) {
                x_mas += 1;
            }
        }

        x_mas
    }

    /// Grab the characters at the vecs, and compare if the resulting parts is "MAS". We started at
    /// X, so we already know the first character to be X.
    fn is_this_xmas(&self, vecs: Vec<Coordinate>) -> bool {
        let mut mas = "".to_string();

        for coordinate in vecs.iter() {
            mas.push(self.word_grid.get(coordinate).unwrap_or(BAD_CHAR).to_owned());
        }

        mas == CHECKMAS
    }

    /// Used with the coordinates we get from generate coordinates part 2, this checks whether
    /// letters surrounding A are MS or SM. To be used with a coordinate pair for either + or x
    /// orientation.
    fn is_this_x_mas(&self, coords: &[[Coordinate; 2]; 2]) -> bool {
        for coord_pair in coords.iter() {
            let mut mas = "".to_string();

            for coordinate in coord_pair {
                mas.push(self.word_grid.get(coordinate).unwrap_or(BAD_CHAR).to_owned());
            }

            if mas != MS && mas != SM {
                return false;
            }
        }

        true
    }
}

fn generate_coordinates_part1(origin: &Coordinate, direction: &Direction) -> Vec<Coordinate> {
    match direction {
        Direction::North => {
            vec![
                (origin.0, origin.1-1),
                (origin.0, origin.1-2),
                (origin.0, origin.1-3),
            ]
        }
        Direction::NorthEast => {
            vec![
                (origin.0+1, origin.1-1),
                (origin.0+2, origin.1-2),
                (origin.0+3, origin.1-3),
            ]
        }
        Direction::East => {
            vec![
                (origin.0+1, origin.1),
                (origin.0+2, origin.1),
                (origin.0+3, origin.1),
            ]
        }
        Direction::SouthEast => {
            vec![
                (origin.0+1, origin.1+1),
                (origin.0+2, origin.1+2),
                (origin.0+3, origin.1+3),
            ]
        }
        Direction::South => {
            vec![
                (origin.0, origin.1+1),
                (origin.0, origin.1+2),
                (origin.0, origin.1+3),
            ]
        }
        Direction::SouthWest => {
            vec![
                (origin.0-1, origin.1+1),
                (origin.0-2, origin.1+2),
                (origin.0-3, origin.1+3),
            ]
        }
        Direction::West => {
            vec![
                (origin.0-1, origin.1),
                (origin.0-2, origin.1),
                (origin.0-3, origin.1),
            ]
        }
        Direction::NorthWest => {
            vec![
                (origin.0-1, origin.1-1),
                (origin.0-2, origin.1-2),
                (origin.0-3, origin.1-3),
            ]
        }
    }
}

/// Return type is a 2-part that groups the relevant pairs of a 2 part array
/// (bottom left - top right) and (top left - bottom right) for x shape.
fn generate_coordinate_pairs_part2(origin: &Coordinate) -> [[Coordinate; 2]; 2] {
   [
       // x shape
       [
           (origin.0-1, origin.1-1), // top left
           (origin.0+1, origin.1+1) // bottom right
       ],
       [
           (origin.0+1, origin.1-1), // top right
           (origin.0-1, origin.1+1) // bottom left
       ]
   ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_coordinates_north() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::North;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0, origin.1-1),
            (origin.0, origin.1-2),
            (origin.0, origin.1-3)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_coordinates_north_east() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::NorthEast;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0+1, origin.1-1),
            (origin.0+2, origin.1-2),
            (origin.0+3, origin.1-3)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_coordinates_east() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::East;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0+1, origin.1),
            (origin.0+2, origin.1),
            (origin.0+3, origin.1)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_coordinates_south_east() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::SouthEast;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0+1, origin.1+1),
            (origin.0+2, origin.1+2),
            (origin.0+3, origin.1+3)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_coordinates_south() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::South;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0, origin.1+1),
            (origin.0, origin.1+2),
            (origin.0, origin.1+3)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_coordinates_south_west() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::SouthWest;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0-1, origin.1+1),
            (origin.0-2, origin.1+2),
            (origin.0-3, origin.1+3)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_coordinates_west() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::West;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0-1, origin.1),
            (origin.0-2, origin.1),
            (origin.0-3, origin.1)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_coordinates_north_west() {
        let origin: Coordinate = (3, 3);
        let direction = Direction::NorthWest;
        let result = generate_coordinates_part1(&origin, &direction);
        let expected = vec![
            (origin.0-1, origin.1-1),
            (origin.0-2, origin.1-2),
            (origin.0-3, origin.1-3)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_finds_xmas_down() {
        let input = concat!(
        "X...\n",
        "M...\n",
        "A...\n",
        "S...\n"
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_xmas_up() {
        let input = concat!(
        "S...\n",
        "A...\n",
        "M...\n",
        "X...\n"
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_xmas_right() {
        let input = concat!(
        "XMAS\n",
        "....\n",
        "....\n",
        "....\n"
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_xmas_left() {
        let input = concat!(
        "SAMX\n",
        "....\n",
        "....\n",
        "....\n"
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_xmas_down_right() {
        let input = concat!(
        "X...\n",
        ".M..\n",
        "..A.\n",
        "...S\n"
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_xmas_down_left() {
        let input = concat!(
        "...X\n",
        "..M.\n",
        ".A..\n",
        "S...\n"
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_xmas_up_left() {
        let input = concat!(
            "S...\n",
            ".A..\n",
            "..M.\n",
            "...X\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_xmas_up_right() {
        let input = concat!(
            "...S\n",
            "..A.\n",
            ".M..\n",
            "X...\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_finds_multiple_xmas() {
        let input = concat!(
        "S..S\n",
        "A.A.\n",
        "MM..\n",
        "XMAS\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_xmas();

        assert_eq!(xmases, 3);
    }

    #[test]
    fn test_part2_example() {
        let input = concat!(
        ".M.S......\n",
        "..A..MSMS.\n",
        ".M.S.MAA..\n",
        "..A.ASMSM.\n",
        ".M.S.M....\n",
        "..........\n",
        "S.S.S.S.S.\n",
        ".A.A.A.A..\n",
        "M.M.M.M.M.\n",
        "..........\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 9);
    }

    #[test]
    fn test_part2_simple_plus() {
        let input = concat!(
        ".M.\n",
        "MAS\n",
        ".S.\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 0);
    }

    #[test]
    fn test_part2_inverted_plus() {
        let input = concat!(
        ".S.\n",
        "SAM\n",
        ".M.\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 0);
    }

    #[test]
    fn test_part2_half_inverted_vert_plus() {
        let input = concat!(
        ".M.\n",
        "SAM\n",
        ".S.\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 0);
    }


    #[test]
    fn test_part2_half_inverted_hor_plus() {
        let input = concat!(
        ".S.\n",
        "MAS\n",
        ".M.\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 0);
    }

    #[test]
    fn test_part2_simple_ex_plus() {
        let input = concat!(
        "MMS\n",
        "MAS\n",
        "MSS\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_part2_simple_ex() {
        let input = concat!(
        "M.S\n",
        ".A.\n",
        "M.S\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_part2_simple_inverted_ex() {
        let input = concat!(
        "S.M\n",
        ".A.\n",
        "M.S\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 0);
    }

    #[test]
    fn test_part2_simple_half_inverted_ex() {
        let input = concat!(
        "S.M\n",
        ".A.\n",
        "S.M\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_part2_simple_other_half_inverted_ex() {
        let input = concat!(
        "M.S\n",
        ".A.\n",
        "M.S\n",
        );

        let d4 = Day04::new(&input);
        let xmases = d4.how_many_x_mas_part2();

        assert_eq!(xmases, 1);
    }
}