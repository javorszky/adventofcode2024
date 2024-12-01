use std::fs;

pub struct Day01 {
    list_left: Vec<i32>,
    list_right: Vec<i32>,
}

impl Day01 {
    pub fn solve(&self) -> String {
        if self.list_right.len() != self.list_left.len() {
            return String::from("the list of location IDs is bad :(");
        }

        let mut total = 0;

        for (i, x) in self.list_left.iter().enumerate() {
            total += (x - self.list_right[i]).abs();
        }

        format!("Total difference is {}", total)
    }

    pub fn new() -> Day01 {
        println!("what is happening");
        let file_contents = fs::read_to_string("./day01/input.txt").unwrap_or("".to_string());
        let trimmed = file_contents.trim();

        let boo: Vec<i32> = trimmed
            .split("\n")
            .flat_map(|line: &str| -> Vec<i32> {
                line.split("   ")
                    .map(|x: &str| -> i32 { x.parse::<i32>().unwrap_or(i32::MAX) })
                    .collect::<Vec<i32>>()
            })
            .collect();

        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();
        for (i, x) in boo.into_iter().enumerate() {
            if i % 2 == 0 {
                left_list.push(x);
            } else {
                right_list.push(x);
            }
        }

        left_list.sort();
        right_list.sort();

        println!("Left list: {:?}", left_list);
        println!("Right list: {:?}", right_list);
        Day01 {
            list_left: left_list,
            list_right: right_list,
        }
    }
}

impl Default for Day01 {
    fn default() -> Self {
        Self::new()
    }
}
