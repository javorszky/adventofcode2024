use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum RuleOrder {
    Before,
    After,
    Unknown
}



pub fn part1() -> i32 {
    solve_part1(include_str!("../input.txt"))
}

pub fn part2() -> i32 {
    solve_part2(include_str!("../input.txt"))
}

fn solve_part1(data: &str) -> i32 {
    let content: Vec<&str> = data.split("\n\n").collect();
    let rules = parse_rules(content[0]);

    let mut middles = 0;

    for line in content[1].lines() {
        let print = parse_print(line);
        if check_print(&print, &rules) {
            middles += get_middle(line);
        }
    }

    middles
}

fn solve_part2(data: &str) -> i32 {
    let content: Vec<&str> = data.split("\n\n").collect();
    let rules = parse_rules(content[0]);

    let mut middles = 0;

    for line in content[1].lines() {
        let print = parse_print(line);
        if !check_print(&print, &rules) {
            let fixed = fix_print(line, &rules);
            middles += get_middle(fixed.as_str());
        }
    }

    middles
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

fn parse_into_numbers(data: &str) -> Vec<i32> {
    data.split(",").map(|part| -> i32 {
        part.parse().unwrap()
    }).collect::<Vec<i32>>()
}

fn parse_print(data: &str) -> HashMap<i32, HashMap<i32, RuleOrder>> {
    let mut local_hm: HashMap<i32, HashMap<i32, RuleOrder>> = HashMap::new();
    let parts = parse_into_numbers(data);

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
}

fn get_middle(data: &str) -> i32 {
    let parts = parse_into_numbers(data);

    parts[(parts.len()-1)/2]
}

fn check_print(print: &HashMap<i32, HashMap<i32, RuleOrder>>, rules: &HashMap<i32, HashMap<i32, RuleOrder>>) -> bool {
    let empty: HashMap<i32, RuleOrder> = HashMap::new();
    for (num, rule) in print.iter() {
        // for every page in our print
        for (other_index, order) in rule.iter() {
            // let's iterate over every relationship it has to the other pages in the print

            // but also let's get what the order should be per the rules, and
            let thing =  rules.get(num).unwrap_or(&empty).get(other_index).unwrap_or(&RuleOrder::Unknown);
            if thing != &RuleOrder::Unknown && thing != order {
                // if it's NOT unknown, OR not the same as what we expect, break as nope
                return false;
            }
        }
    }

    true
}

fn fix_print(data: &str, rules: &HashMap<i32, HashMap<i32, RuleOrder>>) -> String {
    let nums = parse_into_numbers(data);

    let empty: HashMap<i32, RuleOrder> = HashMap::new();

    let mut weights: HashMap<i32, Vec<&RuleOrder>> = HashMap::new();

    // let's see what the relations of all numbers in the print to all other numbers
    for first in nums.clone() {
        weights.insert(first, Vec::new());
        for second in nums.clone() {
            if first == second {
                continue
            }

            let ro = rules.get(&first).unwrap_or(&empty).get(&second).unwrap_or(&RuleOrder::Unknown);

            weights.get_mut(&first).unwrap().push(ro);
        }
    }

    let mut orders: HashMap<i32, i32> = HashMap::new();

    for (num, weight_list) in weights {
        let mut acc = 0;
        for order in weight_list {
            match order {
                RuleOrder::After => { acc -= 1}
                RuleOrder::Before => { acc += 1; }
                RuleOrder::Unknown => {  }
            }
        }

        orders.insert(acc, num);
    }

    let mut k = orders.keys().collect::<Vec<&i32>>();

    k.sort();

    let mut sorted:Vec<&i32> = Vec::new();
    for key in k {
        sorted.push(orders.get(key).unwrap());
    }

    let s = sorted.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(",");

    s
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

        let expected: HashMap<i32, HashMap<i32, RuleOrder>> = HashMap::from([
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
        ]);

        let parsed = parse_print(input);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_check_print_valid() {
        let rules = parse_rules("1|2\n2|4");
        let print = parse_print("1,2,3,4");
        let result = check_print(&print, &rules);

        assert_eq!(result, true);
    }

    #[test]
    fn test_check_print_invalid() {
        let rules = parse_rules("1|2\n2|4");
        let print = parse_print("4,2,1,3");
        let result = check_print(&print, &rules);

        assert_eq!(result, false);
    }

    #[test]
    fn test_get_middle() {
        let input = "1,2,3,4,5";
        let result = get_middle(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13";

        let solution = solve_part1(input);

        assert_eq!(143, solution);
    }

    #[test]
    fn test_fix_prints() {
        let rules_input = "1|2
1|4
1|2
1|3
1|4
2|4
3|2
3|4";
        let print_input = "1,2,3,4,5";


        let rules = parse_rules(rules_input);
        let print = parse_print(print_input);

        assert_eq!(check_print(&print, &rules), false);

        // assert_eq!(1, 0);

        assert_eq!(fix_print(print_input, &rules), "1,3,5,2,4");
    }
}