use crate::history::History;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("../input.txt").expect("Should have been able to read the file");

    fn parse_line(line: &str) -> History {
        let line = line
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect();
        History::new(line)
    }
    let histories: Vec<History> = contents.lines().map(parse_line).collect();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for history in histories {
        let (part1, part2) = history.calc_values();
        sum_part1 += part1;
        sum_part2 += part2;
    }
    println!("Part1: {sum_part1}");
    println!("Part2: {sum_part2}");
}

mod history {
    pub struct History {
        main_line: Vec<i32>,
    }

    impl History {
        pub fn new(line: Vec<i32>) -> History {
            return History {
                main_line: line,
            };
        }

        pub fn calc_values(&self) -> (i32, i32) {
            let mut line_to_calc: Vec<i32> = self.main_line.clone();
            let mut following_lines_first_value: Vec<i32> = Vec::new();
            let mut following_lines_last_value: Vec<i32> = Vec::new();
            following_lines_first_value.push(self.main_line.first().unwrap().clone());
            following_lines_last_value.push(self.main_line.last().unwrap().clone());
            loop {
                let differences = calc_differences(&line_to_calc);
                // println!("differences: {:?}", differences);
                let finished = differences.iter().all(|val| *val == 0);
                following_lines_last_value.push(differences.last().unwrap().clone());
                following_lines_first_value.push(differences.first().unwrap().clone());
                line_to_calc = differences;
                if finished {
                    break;
                }
            }
            let mut after_value: i32 = 0;
            for last_value in following_lines_last_value.iter().rev() {
                after_value += last_value;
            }
            // println!("after value: {after_value}");

            let mut before_value: i32 = 0;
            for first_value in following_lines_first_value.iter().rev() {
                before_value = first_value - before_value;
            }
            // println!("before value: {before_value}");
            (after_value, before_value)
        }
    }

    fn calc_differences(line: &Vec<i32>) -> Vec<i32> {
        let result: (Vec<i32>, Option<&i32>) =
            line.iter().fold((Vec::new(), None), |mut acc, item| {
                if let Some(last) = acc.1 {
                    acc.0.push(item - last);
                };
                (acc.0, Some(item))
            });
        result.0
    }
}
