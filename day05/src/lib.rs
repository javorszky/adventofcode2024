use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq)]
enum RuleOrder {
    Before,
    After
}


struct Day05 {
    rules: HashMap<i32, HashMap<i32, RuleOrder>>,
}

pub fn part1() -> String {
    let content: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let rules = parse_rules(content[0]);

    let _d5 = Day05 { rules };



    "".to_string()
}

fn parse_rules(data: &str) -> HashMap<i32, HashMap<i32, RuleOrder>> {
    let mut rules:HashMap<i32, HashMap<i32, RuleOrder>> = HashMap::new();

    for line in data.lines() {
        let split: Vec<&str> = line.split("|").collect();
        let before = split[0].parse().unwrap();
        let after = split[1].parse().unwrap();

        rules.entry(before).or_insert(HashMap::new()).insert(after, RuleOrder::After);
        rules.entry(after).or_insert(HashMap::new()).insert(before, RuleOrder::Before);
    }

    rules
}

fn parse_prints(data: &str) -> Vec<HashMap<i32, HashMap<i32, RuleOrder>>> {
    let mut line_rules: Vec<HashMap<i32, HashMap<i32, RuleOrder>>> = Vec::new();

    line_rules = data.lines().map(|line| -> HashMap<i32, HashMap<i32, RuleOrder>>{
        let mut local_hm: HashMap<i32, HashMap<i32, RuleOrder>> = HashMap::new();
        let parts = line.split(",").map(|part| -> i32 {
            part.parse().unwrap()
        }).collect::<Vec<i32>>();

        let l = parts.len();

        for (current, number) in parts.iter().enumerate() {
            for other_index in 0..parts.len() {
                if current == other_index {
                    // we're looking at the same number, do nothing
                    continue;
                } else if other_index < current {
                    // other index is _before_ the current one, so
                    local_hm.entry(*number).or_insert(HashMap::new()).insert(parts[other_index], RuleOrder::Before);
                } else {
                    local_hm.entry(*number).or_insert(HashMap::new()).insert(parts[other_index], RuleOrder::After);
                }
            }
        }

        local_hm
    }).collect::<Vec<HashMap<i32, HashMap<i32, RuleOrder>>>>();

    line_rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules_single() {
        let hm = parse_rules("31|42");
        let expected: HashMap<i32, HashMap<i32, RuleOrder>> = HashMap::from([
            (31, HashMap::from([(42, RuleOrder::After)])),
            (42, HashMap::from([(31, RuleOrder::Before)]))
        ]);

        assert_eq!(hm, expected);
    }

    #[test]
    fn test_parse_rules_multiline_no_overlap() {
        let hm = parse_rules("31|42\n99|11");
        let expected: HashMap<i32, HashMap<i32, RuleOrder>> = HashMap::from([
            (31, HashMap::from([(42, RuleOrder::After)])),
            (99, HashMap::from([(11, RuleOrder::After)])),
            (42, HashMap::from([(31, RuleOrder::Before)])),
            (11, HashMap::from([(99, RuleOrder::Before)]))
        ]);

        assert_eq!(hm, expected);
    }

    #[test]
    fn test_parse_rules_multiline_overlaps() {
        let hm = parse_rules("31|42\n99|31\n31|11");
        let expected: HashMap<i32, HashMap<i32, RuleOrder>> = HashMap::from([
            (31, HashMap::from([
                (42, RuleOrder::After),
                (11, RuleOrder::After),
                (99, RuleOrder::Before)
            ])),
            (99, HashMap::from([
                (31, RuleOrder::After)
            ])),
            (42, HashMap::from([
                (31, RuleOrder::Before)
            ])),
            (11, HashMap::from([
                (31, RuleOrder::Before)
            ]))
        ]);

        assert_eq!(hm, expected);
    }

    #[test]
    fn test_parse_prints() {
        let input = "1,2,3,4";

        let expected: Vec<HashMap<i32, HashMap<i32, RuleOrder>>> = vec![
            HashMap::from([
                (1, HashMap::from([
                    (2, RuleOrder::After),
                    (3, RuleOrder::After),
                    (4, RuleOrder::After),
                ])),
                (2, HashMap::from([
                    (1, RuleOrder::Before),
                    (3, RuleOrder::After),
                    (4, RuleOrder::After),
                ])),
                (3, HashMap::from([
                    (1, RuleOrder::Before),
                    (2, RuleOrder::Before),
                    (4, RuleOrder::After),
                ])),
                (4, HashMap::from([
                    (2, RuleOrder::Before),
                    (1, RuleOrder::Before),
                    (3, RuleOrder::Before),
                ])),
            ])
        ];


        // Left:  [{1: {3: After, 2: After}, 2: {1: Before, 3: After}, 3: {2: Before, 1: Before}, 4: {1: Before, 2: Before, 3: Before}}]
        // Right: [{1: {3: After, 2: After, 4: After}, 2: {1: Before, 3: After, 4: After}, 3: {1: Before, 4: After, 2: Before}, 4: {3: Before, 2: Before, 1: Before}}]
        //

        let parsed = parse_prints(input);

        assert_eq!(parsed, expected);
    }
}