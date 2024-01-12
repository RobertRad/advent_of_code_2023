use std::cmp;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut part1_hands = parse_hands(&lines, false);
    part1_hands.sort();
    let mut sum: u32 = 0;
    for (index, hand) in part1_hands.iter().enumerate() {
        let rank = u32::try_from(index + 1).unwrap();
        let value = rank * hand.bid;
        sum += value;
        // println!("[{hand}] has rank {rank}");
    }
    println!("Part1: {sum}");

    let mut part2_hands = parse_hands(&lines, true);
    part2_hands.sort();
    let mut sum: u32 = 0;
    for (index, hand) in part2_hands.iter().enumerate() {
        let rank = u32::try_from(index + 1).unwrap();
        let value = rank * hand.bid;
        sum += value;
        // println!("[{hand}] has rank {rank}");
    }
    println!("Part2: {sum}");
}

fn parse_hands(lines: &Vec<&str>, joker_rule: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in lines {
        hands.push(parse_hand(line, joker_rule));
    }
    hands
}

fn parse_hand(line: &str, joker_rule: bool) -> Hand {
    let split: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(2, split.len());
    let cards: Vec<char> = split[0].chars().collect();
    assert_eq!(5, cards.len());
    let cards = [
        parse_card(cards[0], joker_rule),
        parse_card(cards[1], joker_rule),
        parse_card(cards[2], joker_rule),
        parse_card(cards[3], joker_rule),
        parse_card(cards[4], joker_rule)
        ];
    let bid = split[1].parse::<u32>().unwrap();
    Hand {
        cards,
        bid,
        joker_rule
    }
}

#[derive(Eq, Ord, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    fn rank(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl cmp::PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}

impl cmp::PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

#[derive(Eq, Ord, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    joker_rule: bool
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut map = BTreeMap::new();
        for card in &self.cards {
            map.entry(card)
            .and_modify(|e| { *e += 1})
            .or_insert(1);
        }
        let joker_count = map.remove(&Card::Joker).unwrap_or(0);
        let max_count = map.values().max().unwrap_or(&0);
        let max_card = map.iter().find(|e| e.1 == max_count).map(|e| *e.0);

        let max_count =  max_count + joker_count;
        match max_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                let also_one_pair_found = map.iter().any(|e| *e.0 != max_card.unwrap() && *e.1 == 2);
                if also_one_pair_found { HandType::FullHouse } else { HandType::ThreeOfAKind }
            },
            2 => {
                if map.values().filter(|v| **v == 2).count() == 2 { HandType::TwoPair } else { HandType::OnePair }
            },
            1 => HandType::HighCard,
            _ => panic!("More than five cards found :O")
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{} {}", self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4], self.bid)
    }
}

impl cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        (&self.hand_type(), &self.cards).partial_cmp(&(&other.hand_type(), &other.cards))
    }
}

#[derive(Eq, Ord, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker
}

impl Card {
    fn strength(&self) -> i32 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Joker => 1,
        }
    }

    fn symbol(&self) -> char {
        match self {
            Card::Ace => 'A',
            Card::King => 'K',
            Card::Queen => 'Q',
            Card::Jack => 'J',
            Card::Ten => 'T',
            Card::Nine => '9',
            Card::Eight => '8',
            Card::Seven => '7',
            Card::Six => '6',
            Card::Five => '5',
            Card::Four => '4',
            Card::Three => '3',
            Card::Two => '2',
            Card::Joker => 'J',
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

impl cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.strength().partial_cmp(&other.strength())
    }
}

fn parse_card(symbol: char, joker_rule: bool) -> Card {
    match symbol {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => if joker_rule { Card::Joker } else { Card::Jack },
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        unknown => panic!("Unknown symbol: {unknown}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_type_without_jokers() {
        let result = parse_hand("TJQKA 1", false).hand_type();
        assert_eq!(HandType::HighCard, result);

        let result = parse_hand("JQKAA 2", false).hand_type();
        assert_eq!(HandType::OnePair, result);

        let result = parse_hand("QKKAA 3", false).hand_type();
        assert_eq!(HandType::TwoPair, result);

        let result = parse_hand("QKAAA 4", false).hand_type();
        assert_eq!(HandType::ThreeOfAKind, result);

        let result = parse_hand("KAAAA 5", false).hand_type();
        assert_eq!(HandType::FourOfAKind, result);

        let result = parse_hand("AAAAA 6", false).hand_type();
        assert_eq!(HandType::FiveOfAKind, result);
    }

    #[test]
    fn hand_type_with_jokers() {
        let result = parse_hand("9TQKA 1", true).hand_type();
        assert_eq!(HandType::HighCard, result);

        let result = parse_hand("TTQKA 0", true).hand_type();
        assert_eq!(HandType::OnePair, result);

        let result = parse_hand("TJQKA 0", true).hand_type();
        assert_eq!(HandType::OnePair, result);

        let result = parse_hand("QKKAA 0", false).hand_type();
        assert_eq!(HandType::TwoPair, result);

        let result = parse_hand("QKAAA 0", false).hand_type();
        assert_eq!(HandType::ThreeOfAKind, result);

        let result = parse_hand("QKAAJ 0", true).hand_type();
        assert_eq!(HandType::ThreeOfAKind, result);

        let result = parse_hand("KKQQQ 0", true).hand_type();
        assert_eq!(HandType::FullHouse, result);

        let result = parse_hand("KKQQJ 0", true).hand_type();
        assert_eq!(HandType::FullHouse, result);

        let result = parse_hand("KAAAA 0", true).hand_type();
        assert_eq!(HandType::FourOfAKind, result);

        let result = parse_hand("KAAAJ 0", true).hand_type();
        assert_eq!(HandType::FourOfAKind, result);

        let result = parse_hand("AAAAA 0", true).hand_type();
        assert_eq!(HandType::FiveOfAKind, result);

        let result = parse_hand("AAAJJ 0", true).hand_type();
        assert_eq!(HandType::FiveOfAKind, result);

        let result = parse_hand("JJJJJ 0", true).hand_type();
        assert_eq!(HandType::FiveOfAKind, result);
    }
}
