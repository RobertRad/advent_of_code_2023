use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let max_colors = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let lines = contents.lines();
    let mut sum: usize = 0;
    let mut sum_power: u32 = 0;

    for line in lines {
        let mut game_possible = true;
        let mut minimum_blue = 0;
        let mut minimum_red = 0;
        let mut minimum_green = 0;

        let colon_index = line.find(':').unwrap();
        let game_number = &line["Game ".len()..colon_index];
        let game_number = game_number.parse::<usize>().unwrap();
        let game_info = &line[colon_index + 2..];
        let subsets = game_info.split(';');
        for subset in subsets {
            let revealed_items = subset.split(",");
            for revealed_item in revealed_items {
                let revealed_item = revealed_item.trim();
                let space_index = revealed_item.find(' ').unwrap();
                let number = &revealed_item[0..space_index];
                let color = &revealed_item[space_index + 1..];
                let number = number.parse::<u32>().unwrap();
                match color {
                    "red" => minimum_red = cmp::max(minimum_red, number),
                    "green" => minimum_green = cmp::max(minimum_green, number),
                    "blue" => minimum_blue = cmp::max(minimum_blue, number),
                    _ => panic!("Unknown color: {color}")
                }
                let max_for_color = max_colors.get(color).unwrap();
                let revealed_item_possible = number <= *max_for_color;
                if !revealed_item_possible {
                    game_possible = false;
                }
            }
        }
        if game_possible {
            sum += game_number;
        }
        let power = minimum_red * minimum_green * minimum_blue;
        sum_power += power;
    }
    println!("Sum: {sum}");
    println!("Sum power: {sum_power}");
}
