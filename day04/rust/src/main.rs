use regex::Regex;
use std::collections::BTreeSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let cards = parse(&lines);
    part1(&cards);
    part2(&cards);
}

struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn new(winning_numbers: Vec<u32>, my_numbers: Vec<u32>) -> Card {
        return Card { winning_numbers, my_numbers }
    }
}

fn parse(lines: &Vec<&str>) -> Vec<Card> {
    let start_regex = Regex::new("Card +\\d+: ").unwrap();
    let mut result: Vec<Card> = Vec::new();
    for line in lines {
        let begin_content = start_regex.find(line).unwrap().end();
        let splitted = line[begin_content..].split("|").collect::<Vec<&str>>();

        fn extract_numbers(string: &Vec<&str>, index: usize) -> Vec<u32> {
            let numbers = string.get(index).unwrap();
            let numbers: Vec<&str> = numbers.split_whitespace().collect();
            let numbers: Vec<u32> = numbers.iter().map(|num| num.parse::<u32>().unwrap()).collect();
            numbers
        }

        let winning_numbers = extract_numbers(&splitted, 0);
        let my_numbers = extract_numbers(&splitted, 1);
        result.push(Card::new(winning_numbers, my_numbers));
    }
    result
}

fn part1(cards: &Vec<Card>) {
    let mut sum: u32 = 0;
    for card in cards {
        let winning_numbers: BTreeSet<u32>= BTreeSet::from_iter(card.winning_numbers.iter().cloned());
        let mut match_count = 0;
        for number in card.my_numbers.iter().clone() {
            if winning_numbers.contains(&number) {
                match_count += 1;
            }
        }
        let value = if match_count == 0 { 0 } else { 2_u32.pow(match_count - 1) };
        sum += value;
    }
    println!("Part1: {sum}");
}

fn part2(cards: &Vec<Card>) {
    let mut sum: u32 = 0;
    let mut number_of_scratchcards: Vec<u32> = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let winning_numbers: BTreeSet<u32>= BTreeSet::from_iter(card.winning_numbers.iter().cloned());
        let mut match_count = 0;
        for number in card.my_numbers.iter().clone() {
            if winning_numbers.contains(&number) {
                match_count += 1;
            }
        }
        let current_number_of_scratchcards = number_of_scratchcards.get(index).unwrap().clone();
        for other_card_index in (index + 1)..=(index + match_count) {
            if let Some(elem) = number_of_scratchcards.get_mut(other_card_index) {
                *elem += current_number_of_scratchcards;
            }
        }
        let value = number_of_scratchcards.get(index).unwrap();
        // println!("Card {}: {value}", (index + 1));
        sum += value
    }
    println!("Part2: {sum}");
}
