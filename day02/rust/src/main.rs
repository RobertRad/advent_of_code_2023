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

    for line in lines {
        let mut game_possible = true;

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
    }
    println!("Sum: {sum}");
}
