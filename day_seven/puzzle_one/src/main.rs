#![feature(slice_flatten)]
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
#[allow(dead_code)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: PossibleHands,
    hand_map: HashMap<char, u32>,
    position_value_map: Vec<u32>,
}

#[derive(Debug)]
enum PossibleHands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let lines = read_to_string("../aoc_day_7").expect("No File Found");

    let mut five_of_a_kind: Vec<Hand> = Vec::new();
    let mut four_of_a_kind: Vec<Hand> = Vec::new();
    let mut full_house: Vec<Hand> = Vec::new();
    let mut three_of_a_kind: Vec<Hand> = Vec::new();
    let mut two_pair: Vec<Hand> = Vec::new();
    let mut one_pair: Vec<Hand> = Vec::new();
    let mut high_card: Vec<Hand> = Vec::new();

    for line in lines.lines() {
        let mut sections = line.split_whitespace();
        let hand = sections.next().unwrap();
        let bid = sections.next().unwrap().parse::<i32>().unwrap();

        let mut position_value_map = vec![0; 5];

        let mut hand_map = HashMap::new();

        for (i, c) in hand.chars().enumerate() {
            hand_map
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            position_value_map[i] = match c {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                _ => 0,
            };
        }

        for (i, value) in position_value_map.iter().enumerate() {
            println!("{}: {}", i, value);
        }

        match hand_map.len() {
            5 => {
                high_card.push(Hand {
                    cards: hand.to_string(),
                    bid: bid as u32,
                    hand_type: PossibleHands::HighCard,
                    position_value_map,
                    hand_map,
                });
            }
            4 => {
                one_pair.push(Hand {
                    cards: hand.to_string(),
                    bid: bid as u32,
                    hand_type: PossibleHands::OnePair,
                    position_value_map,
                    hand_map,
                });
            }
            3 => {
                if hand_map.values().any(|&x| x == 3) {
                    three_of_a_kind.push(Hand {
                        cards: hand.to_string(),
                        bid: bid as u32,
                        hand_type: PossibleHands::ThreeOfAKind,
                        position_value_map,
                        hand_map,
                    });
                } else {
                    two_pair.push(Hand {
                        cards: hand.to_string(),
                        bid: bid as u32,
                        hand_type: PossibleHands::TwoPair,
                        position_value_map,
                        hand_map,
                    });
                }
            }
            2 => {
                if hand_map.values().any(|&x| x == 4) {
                    four_of_a_kind.push(Hand {
                        cards: hand.to_string(),
                        bid: bid as u32,
                        hand_type: PossibleHands::FourOfAKind,
                        position_value_map,
                        hand_map,
                    });
                } else {
                    full_house.push(Hand {
                        cards: hand.to_string(),
                        bid: bid as u32,
                        hand_type: PossibleHands::FullHouse,
                        position_value_map,
                        hand_map,
                    });
                }
            }
            _ => {
                five_of_a_kind.push(Hand {
                    cards: hand.to_string(),
                    bid: bid as u32,
                    hand_type: PossibleHands::FiveOfAKind,
                    position_value_map,
                    hand_map,
                });
            }
        };
    }
}
