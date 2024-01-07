use std::fmt;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let races_part1 = parse_part1(&lines).unwrap();
    let part1 = solve(races_part1);
    println!("Part1: {part1}");

    let race_part2 = parse_part2(&lines).unwrap();
    let part2 = solve(vec![race_part2]);
    println!("Part2: {part2}");
}

fn solve(races: Vec<Race>) -> i32 {
    let mut count_winning_ways = 1;
    for race in races {
        let mut count_winning_ways_for_race = 0;
        for millis in 1..race.time {
            let speed = millis;
            let distance = speed * (race.time - millis);
            if distance > race.distance {
                count_winning_ways_for_race += 1;
            }
        }
        println!("{race}, winning ways: {count_winning_ways_for_race}");
        count_winning_ways *= count_winning_ways_for_race;
    }
    count_winning_ways
}

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn new(time: i64, distance: i64) -> Race {
        Race{ time, distance}
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Race(time: {}, distance: {})", self.time, self.distance)
    }
}

fn parse_part1(lines: &Vec<&str>) -> Option<Vec<Race>> {
    let time_line = lines.get(0)?;
    let times: Vec<i64> = time_line.split_whitespace().skip(1).map(|t| t.parse::<i64>().unwrap()).collect();
    let distance_line = lines.get(1)?;
    let distances: Vec<i64> = distance_line.split_whitespace().skip(1).map(|d| d.parse::<i64>().unwrap()).collect();
    if times.len() != distances.len() {
        panic!("Number of times = {}, number of distances: {}", times.len(), distances.len());
    }
    let mut result = Vec::new();
    for i in 0..times.len() {
        result.push(Race::new(times[i], distances[i]));
    }
    Some(result)
}

fn parse_part2(lines: &Vec<&str>) -> Option<Race> {
    let time_line = lines.get(0)?;
    fn parse_line(line: &str) -> i64 {
        let ignored_spaces = line.split_whitespace().skip(1).into_iter().fold(String::new(), |acc, x| acc + x);
        ignored_spaces.parse::<i64>().unwrap()
    }
    let time = parse_line(time_line);
    let distance_line = lines.get(1)?;
    let distance = parse_line(distance_line);
    Some(Race::new(time, distance))
}
