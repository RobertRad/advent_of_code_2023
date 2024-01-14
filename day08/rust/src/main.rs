use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let directions: Vec<Direction> = lines[0].chars().map(|c| {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            other => panic!("Unknown direction: {other}"),
        }
    }).collect();
    // println!("Directions: {:?}", directions);
    let regex = Regex::new(r"(?<key>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
    let mut map = HashMap::new();
    for line in lines.iter().skip(2) {
        let captures = regex.captures(line).unwrap();
        let key = captures.name("key").unwrap().as_str();
        let left = captures.name("left").unwrap().as_str();
        let right = captures.name("right").unwrap().as_str();
        let node = Node { key, left, right };
        map.insert(key, node);
    }
    let mut steps = 0;
    let mut current_area = "AAA";
    loop {
        let index = steps % directions.len();
        let direction = directions.get(index).unwrap();
        let current_node = map.get(current_area).unwrap();
        current_area = match direction {
            Direction::Left => current_node.left,
            Direction::Right => current_node.right,
        };
        steps += 1;
        // println!("Step {steps}: {current_node}");
        if current_area == "ZZZ" {
            break;
        }
    }
    println!("Took {steps} steps to find target.");
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

struct Node<'a> {
    key: &'a str,
    left: &'a str,
    right: &'a str
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: left: {}, right: {}", self.key, self.left, self.right)
    }
}
