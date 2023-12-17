use std::fs;

fn main() {
    let contents = fs::read_to_string("../test.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut sum: u32 = 0;

    for (index, line) in lines.iter().enumerate() {
        let mut start_at = 0;
        loop {
            let number = find_number(line, start_at);
            match number {
                None => break,
                Some(number) => {
                    start_at = number.end_index;
                    let symbol_before = if number.start_index == 0 { '.' } else { line.chars().nth(number.start_index - 1).unwrap() };
                    let symbol_after = if number.end_index == line.len() { '.' } else { line.chars().nth(number.end_index).unwrap() };
                    let symbol_before_or_after = symbol_before != '.' || symbol_after != '.';
                    let previous_line = if index > 0 { Some(lines[index - 1]) } else { None };
                    let has_symbol_in_previous_line = match previous_line {
                        None => false,
                        Some(previous_line) => has_symbol(previous_line, number.start_index, number.end_index),
                    };
                    let next_line = if index < lines.len() - 1 { Some(lines[index + 1]) } else { None };
                    let has_symbol_in_next_line = match next_line {
                        None => false,
                        Some(next_line) => has_symbol(next_line, number.start_index, number.end_index),
                    };
                    let symbol_in_over_or_below = has_symbol_in_previous_line || has_symbol_in_next_line;
                    let symbol_adjacent = symbol_before_or_after || symbol_in_over_or_below;
                    if symbol_adjacent {
                        sum += number.number;
                    }
                    // println!("Line [{index}]: {line} - {}, {symbol_adjacent}", number.number);
                }
            };
        }
    }
    println!("Sum: {sum}");
}

#[derive(Debug)]
struct FoundNumber {
    number: u32,
    start_index: usize,
    end_index: usize,
}

fn find_number(line: &str, start_at: usize) -> Option<FoundNumber> {
    let sub = &line[start_at..];
    let start_index = sub.find(char::is_numeric);
    match start_index {
        None => None,
        Some(start_index) => {
            let start_index = start_index + start_at;
            let sub = &line[start_index..];
            let len = sub.find(|c: char| !c.is_numeric()).unwrap_or_else(|| sub.len());
            let end_index = start_index + len;
            let number = &sub[..len];
            let number = number.parse::<u32>().unwrap();
            Some(FoundNumber {number, start_index, end_index })
        }
    }
}

fn has_symbol(line: &str, start_index: usize, end_index: usize) -> bool {
    let start_index = if start_index == 0 { 0 } else { start_index - 1};
    let end_index = if end_index == line.len() { end_index } else { end_index + 1};
    let sub = &line[start_index..end_index];
    let result = sub.contains(|c: char| !c.is_numeric() && c != '.' );
    result
}
