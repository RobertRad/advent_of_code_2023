use regex::Regex;
use std::collections::BTreeSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut sum: u32 = 0;
    let start_regex = Regex::new("Card +\\d+: ").unwrap();
    for line in lines {
        let begin_content = start_regex.find(line).unwrap().end();
        let splitted = line[begin_content..].split("|").collect::<Vec<&str>>();

        fn extract_numbers(string: &Vec<&str>, index: usize) -> Vec<u32> {
            let numbers = string.get(index).unwrap();
            let numbers: Vec<&str> = numbers.split_whitespace().collect();
            let numbers: Vec<u32> = numbers.iter().map(|num| num.parse::<u32>().unwrap()).collect();
            numbers
        }

        let winning_numbers = extract_numbers(&splitted, 0);
        let my_numbers = extract_numbers(&splitted, 1);
        // println!("winning_numbers: {:?}", winning_numbers);
        // println!("my_numbers: {:?}", my_numbers);
        let winning_numbers: BTreeSet<u32>= BTreeSet::from_iter(winning_numbers.iter().cloned());
        let mut match_count = 0;
        for number in my_numbers {
            if winning_numbers.contains(&number) {
                match_count += 1;
            }
        }
        let value = if match_count == 0 { 0 } else { 2_u32.pow(match_count - 1) };
        sum += value;
    }
    println!("Part1: {sum}");
}
