use std::fs;

fn main() {
    let contents = fs::read_to_string("../test.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    for (index, line) in lines.iter().enumerate() {
        println!("Line [{index}]: {line}");
    }
}