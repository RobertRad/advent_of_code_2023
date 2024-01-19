use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut sum: u32 = 0;

    for (index, line) in lines.iter().enumerate() {
        let mut start_at = 0;
        loop {
            if let Some(number) = find_number(line, start_at) {
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
            } else {
                break;
            }
        }
    }
    println!("Part1: {sum}");
}

fn find_number(line: &str, start_at: usize) -> Option<FoundNumber> {
    let sub = &line[start_at..];
    if let Some(start_index) = sub.find(char::is_numeric) {
        let start_index = start_index + start_at;
        let sub = &line[start_index..];
        let len = sub.find(|c: char| !c.is_numeric()).unwrap_or_else(|| sub.len());
        let end_index = start_index + len;
        let number = &sub[..len];
        let number = number.parse::<u32>().unwrap();
        Some(FoundNumber {number, start_index, end_index })
    } else {
        None
    }
}

fn has_symbol(line: &str, start_index: usize, end_index: usize) -> bool {
    let start_index = if start_index == 0 { 0 } else { start_index - 1};
    let end_index = if end_index == line.len() { end_index } else { end_index + 1};
    let sub = &line[start_index..end_index];
    let result = sub.contains(|c: char| !c.is_numeric() && c != '.' );
    result
}

#[derive(Debug)]
struct FoundNumber {
    number: u32,
    start_index: usize,
    end_index: usize,
}

fn part2(lines: &Vec<&str>) {
    let mut sum: u32 = 0;
    let mut found_gears: HashMap<CharWithPosition, u32> = HashMap::new();

    for (index, line) in lines.iter().enumerate() {
        let mut start_at = 0;
        loop {
            if let Some(found_number) = find_number(line, start_at) {
                start_at = found_number.end_index;
                let number = found_number.number;
                let neighbour_symbol = get_neighbour_symbol(found_number, line, index, lines);
                // println!("Line [{index}]: {line} - {number}, {:?}", neighbour_symbol);
                if let Some(neighbour_symbol) = neighbour_symbol.and_then(|s| if s.c == '*' { Some(s) } else { None }) {
                    if let Some(previous_number) = found_gears.get(&neighbour_symbol) {
                        sum += number * previous_number;
                    } else {
                        found_gears.insert(neighbour_symbol, number);
                    }
                }
            } else {
                break;
            }
        }
    }
    println!("Part2: {sum}");
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct CharWithPosition {
    c: char,
    x: usize,
    y: usize,
}

fn get_neighbour_symbol(number: FoundNumber, line: &str, y: usize, lines: &Vec<&str>) -> Option<CharWithPosition> {
    let symbol_before = if number.start_index == 0 { None } else { line.chars().nth(number.start_index - 1) };
    let symbol_before = if symbol_before == Some('.') { None } else { symbol_before };
    let symbol_before = symbol_before.map(|c| CharWithPosition{c, x: number.start_index - 1, y} );
    let symbol_after = if number.end_index == line.len() { None } else { line.chars().nth(number.end_index) };
    let symbol_after = if symbol_after == Some('.') { None } else { symbol_after };
    let symbol_after = symbol_after.map(|c| CharWithPosition{c, x: number.end_index, y} );
    let previous_line = if y > 0 { Some(lines[y - 1]) } else { None };
    let symbol_in_previous_line = match previous_line {
        None => None,
        Some(previous_line) => find_symbol(previous_line, y - 1, number.start_index, number.end_index),
    };
    let next_line = if y < lines.len() - 1 { Some(lines[y + 1]) } else { None };
    let symbol_in_next_line = match next_line {
        None => None,
        Some(next_line) => find_symbol(next_line, y + 1, number.start_index, number.end_index),
    };
    symbol_before.or(symbol_after).or(symbol_in_previous_line).or(symbol_in_next_line)
}

fn find_symbol(line: &str, y: usize, start_index: usize, end_index: usize) -> Option<CharWithPosition> {
    let start_index = if start_index == 0 { 0 } else { start_index - 1};
    let end_index = if end_index == line.len() { end_index } else { end_index + 1};
    let sub = &line[start_index..end_index];
    let symbol_index = sub.find(|c: char| !c.is_numeric() && c != '.');
    match symbol_index {
        None => None,
        Some(symbol_index) => {
            let c = sub.chars().nth(symbol_index).unwrap();
            Some(CharWithPosition { c, x: start_index + symbol_index, y })
        },
    }
}
