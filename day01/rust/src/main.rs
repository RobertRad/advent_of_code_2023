use std::collections::HashMap;
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
        let first_digit = find_digit(line, part, false);
        let last_digit = find_digit(line, part, true);
        
        let mut number = String::new();
        number.push_str(&first_digit);
        number.push_str(&last_digit);
        
        let number = number.parse::<u32>().unwrap();
        // println!("Line: {line} - found: {number}");
        sum += number;
    }
    println!("{:?}: {sum}", *part);
}

fn find_digit(line: &str, part: &Part, reverse_search: bool) -> String {
    let mut tokens: HashMap<String, String> = HashMap::new();
    for i in 0..10 {
        let val = String::from(i.to_string());
        tokens.insert(val.clone(), val);
    }
    match part {
        Part::Part2 => {
            tokens.insert("one".to_string(), "1".to_string());
            tokens.insert("two".to_string(), "2".to_string());
            tokens.insert("three".to_string(), "3".to_string());
            tokens.insert("four".to_string(), "4".to_string());
            tokens.insert("five".to_string(), "5".to_string());
            tokens.insert("six".to_string(), "6".to_string());
            tokens.insert("seven".to_string(), "7".to_string());
            tokens.insert("eight".to_string(), "8".to_string());
            tokens.insert("nine".to_string(), "9".to_string());
        }
        _ => {}
    }

    let mut first_index: usize = line.len();
    let mut first_token: Option<&str> = None;
    let mut last_index: usize = 0;
    let mut last_token: Option<&str> = None;
    for token in tokens.keys() {
        let index = if reverse_search { line.rfind(token) } else { line.find(token) };
        if index != None {
            let index = index.unwrap();
            if index < first_index {
                first_index = index;
                first_token = Some(token);
            }
            if index >= last_index {
                last_index = index;
                last_token = Some(token);
            }
        }
    }
    let found_digit = if reverse_search { last_token.unwrap() } else { first_token.unwrap() };
    let result = tokens.get(found_digit).unwrap();
    String::from(result)
}

#[derive(Debug)]
enum Part {
    Part1,
    Part2
}
