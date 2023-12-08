#![feature(slice_flatten)]
use itertools::Itertools;
use std::ops::Deref;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
#[allow(dead_code)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: HandType,
    hand_map: HashMap<char, u32>,
    position_value_map: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HandType {
    HighCard = 6,
    OnePair = 5,
    TwoPair = 4,
    ThreeOfAKind = 3,
    FullHouse = 2,
    FourOfAKind = 1,
    FiveOfAKind = 0,
}

fn main() {
    let lines = read_to_string("../aoc_day_7").expect("No File Found");

    fn score_hand(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
        use HandType::*;

        let counts = hand.chars().counts();
        let values = counts.values().sorted().join("");

        let hand_type = match values.deref() {
            "5" => FiveOfAKind,
            "14" => FourOfAKind,
            "23" => FullHouse,
            "113" => ThreeOfAKind,
            "122" => TwoPair,
            "1112" => OnePair,
            "11111" => HighCard,
            value => panic!("Something went wrong. Encountered `{}`", value),
        };

        let card_scores_vec: Vec<u32> = hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                num => num.to_digit(10).unwrap(),
            })
            .collect();

        (
            hand_type,
            (
                card_scores_vec[0],
                card_scores_vec[1],
                card_scores_vec[2],
                card_scores_vec[3],
                card_scores_vec[4],
            ),
        )
    }

    let sum = lines
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid.parse::<u32>().unwrap(), score_hand(hand))
        })
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
        .enumerate()
        .map(|(index, (_hand, bid, _))| (index as u32 + 1) * bid)
        .sum::<u32>();

    println!("Sum: {}", sum);
}
