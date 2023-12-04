use regex::Regex;
use std::fs;

fn main() {
    solve(&Part::Part1);
    solve(&Part::Part2);
}

fn solve(part: &Part) {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines = contents.lines();
    let mut sum: u32 = 0;

    for line in lines {
        let first_digit = find_first_digit(line, part, false);
        let last_digit = find_first_digit(line, part, true);
        
        let mut number = String::new();
        number.push_str(&first_digit);
        number.push_str(&last_digit);
        
        let number = number.parse::<u32>().unwrap();
        // println!("Line: {line} - found: {number}");
        sum += number;
    }
    println!("{:?}: {sum}", *part);
}

fn find_first_digit(line: &str, part: &Part, reverse_search: bool) -> String {
    let regex = match part {
        Part::Part1 =>  Regex::new(r"[0-9]"),
        Part::Part2 => Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine")
    }.unwrap();
    let matches: Vec<_> = regex.find_iter(line).map(|m| m.as_str()).collect();
    let match_index = if reverse_search { matches.len() - 1 } else { 0 };
    let first_digit = matches.get(match_index).expect("No digit found!");
    let result = match *first_digit {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        other => other
    };
    String::from(result)
}

#[derive(Debug)]
enum Part {
    Part1,
    Part2
}
