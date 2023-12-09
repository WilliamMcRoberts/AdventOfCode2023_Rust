use itertools::{Itertools, Position};
use std::fs::read_to_string;
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn main() {
    let lines = read_to_string("../aoc_day_7").expect("No File Found");
    let sum = process(&lines).unwrap();

    println!("Sum: {}", sum);
}

fn score_hand(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = hand.chars().counts();
    let values = match counts.get(&'J') {
        Some(j_count) if *j_count == 5 => "5".to_string(),
        Some(j_count) if *j_count != 5 => counts
            .iter()
            .filter_map(|(key, value)| (key != &'J').then_some(value))
            .sorted()
            .with_position()
            .map(|(pos, value)| match pos {
                Position::Last | Position::Only => value + j_count,
                _ => *value,
            })
            .join(""),
        _ => counts.values().sorted().join(""),
    };
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("No hand was discovered with `{}`", value),
    };
    let card_scores = hand
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 1,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
}

pub fn process(input: &str) -> Result<String, String> {
    let hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand_bid_scores = (hand, bid.parse::<u32>().unwrap(), score_hand(hand));
            println!("Hand Type: {:?}", hand_bid_scores.2 .0);
            println!("Scores: {:?}", hand_bid_scores.2 .1);
            hand_bid_scores
        })
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
        .enumerate()
        .map(|(index, (_, bid, _))| (index as u32 + 1) * bid)
        .sum::<u32>();
    Ok(hands.to_string())
}
