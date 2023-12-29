use std::collections::BTreeMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines);
}

#[derive(Debug)]
enum BoundaryType {
    From,
    To,
    FromTo,
}

#[derive(Debug)]
struct Range {
    bondary_type: BoundaryType,
    addend: i64,
}

fn part1(lines: &Vec<&str>) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut seed_to_soil: BTreeMap<i64, Range> = BTreeMap::new();
    let mut soil_to_fertilizer: BTreeMap<i64, Range> = BTreeMap::new();
    let mut fertilizer_to_water: BTreeMap<i64, Range> = BTreeMap::new();
    let mut water_to_light: BTreeMap<i64, Range> = BTreeMap::new();
    let mut light_to_temperature: BTreeMap<i64, Range> = BTreeMap::new();
    let mut temperature_to_humidity: BTreeMap<i64, Range> = BTreeMap::new();
    let mut humidity_to_location: BTreeMap<i64, Range> = BTreeMap::new();
    let mut current_map = "";
    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.starts_with("seeds: ") {
            let sub: String = line.chars().skip("seeds: ".len()).collect();
            sub.split(" ").for_each(|s| {
                let seed = s.parse::<i64>().unwrap();
                seeds.push(seed);
            });
        } else if line.contains("map") {
            current_map = line;
        } else {
            let split: Vec<&str> = line.split(" ").collect();
            let dest_start = split[0].parse::<i64>().unwrap();
            let source_start = split[1].parse::<i64>().unwrap();
            let len = split[2].parse::<i64>().unwrap();
            let source_end = source_start + len - 1;
            let addend: i64 = dest_start - source_start;
            let start_range = Range { bondary_type: if len != 1 { BoundaryType::From } else { BoundaryType::FromTo }, addend };
            let end_range = Range { bondary_type:  if len != 1 { BoundaryType::To } else { BoundaryType::FromTo }, addend };
            match current_map {
                "seed-to-soil map:" => {
                    seed_to_soil.insert(source_start, start_range);
                    seed_to_soil.insert(source_end , end_range);
                },
                "soil-to-fertilizer map:" => {
                    soil_to_fertilizer.insert(source_start, start_range);
                    soil_to_fertilizer.insert(source_end , end_range);
                 },
                "fertilizer-to-water map:" => {
                    fertilizer_to_water.insert(source_start, start_range);
                    fertilizer_to_water.insert(source_end , end_range);
                },
                "water-to-light map:" => {
                    water_to_light.insert(source_start, start_range);
                    water_to_light.insert(source_end , end_range);
                },
                "light-to-temperature map:" => {
                    light_to_temperature.insert(source_start, start_range);
                    light_to_temperature.insert(source_end , end_range);
                },
                "temperature-to-humidity map:" => {
                    temperature_to_humidity.insert(source_start, start_range);
                    temperature_to_humidity.insert(source_end , end_range);
                },
                "humidity-to-location map:" => {
                    humidity_to_location.insert(source_start, start_range);
                    humidity_to_location.insert(source_end , end_range);
                },
                _ => panic!("Unknown map!")
            }
        }
    }
    fn read_from_map(map: &BTreeMap<i64, Range>, value: &i64) -> i64 {
        let range = map.range(value..).next();
        let result = match range {
            None => value.clone(),
            Some(range) => {
                let direct = value.clone();
                let associated = value.clone() + range.1.addend;
                if range.0 == value {
                    associated
                } else {
                    match range.1.bondary_type {
                        BoundaryType::From => direct,
                        BoundaryType::To => associated,
                        BoundaryType::FromTo => direct,
                    }
                }
            }
        };
        result
    }
    let mut lowest_location: i64 = std::i64::MAX;
    for seed in &seeds {
        let soil = read_from_map(&seed_to_soil, seed);
        let fertilizer = read_from_map(&soil_to_fertilizer, &soil);
        let water = read_from_map(&fertilizer_to_water, &fertilizer);
        let light = read_from_map(&water_to_light, &water);
        let temperature = read_from_map(&light_to_temperature, &light);
        let humidity = read_from_map(&temperature_to_humidity, &temperature);
        let location = read_from_map(&humidity_to_location, &humidity);
        if location < lowest_location {
            lowest_location = location;
        }
    }
    println!("Part1: {lowest_location}");
}
