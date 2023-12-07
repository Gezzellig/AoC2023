use std::cmp::Reverse;
use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::mpsc::channel;

const HAND_SIZE: usize = 5;

struct Hand {
    cards: [char; HAND_SIZE],
    ranking: u64,
    bid: u64,
}

impl FromStr for Hand {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let bid = bid_str.parse().unwrap();
        let mut cards = ['?'; HAND_SIZE];
        for (i, c) in cards_str.chars().enumerate() {
            cards[i] = c;
        }
        let ranking = calc_hand_score(&cards)*10000000000
            + card_to_value(cards[0]) * 100000000
            + card_to_value(cards[1]) * 1000000
            + card_to_value(cards[2]) * 10000
            + card_to_value(cards[3]) * 100
            + card_to_value(cards[4]);

        println!("{}, {} {}",cards_str, ranking, bid);
        Ok(Hand { cards, ranking, bid })
    }
}

fn card_to_value(c:char) -> u64 {
    match c {
        '2' => 12,
        '3' => 13,
        '4' => 14,
        '5' => 15,
        '6' => 16,
        '7' => 17,
        '8' => 18,
        '9' => 19,
        'T' => 20,
        'J' => 21,
        'Q' => 22,
        'K' => 23,
        'A' => 24,
        _ => {
            println!("huuhhhh2");
            0
        }

    }
}

fn calc_hand_score(cards: &[char; 5]) -> u64 {
    let mut card_histogram = [0; 13];
    for card in cards {
        card_histogram[(card_to_value(*card)-12) as usize] += 1;
    }
    let mut max_val = 0;
    let mut max_2val = 0;

    for val in card_histogram {
        if val >= max_val {
            max_2val = max_val;
            max_val = val;
        }
    }
    if max_val == 3 {
        if card_histogram.contains(&2) {
            max_2val = 2
        }
    }
    // let max_val = card_histogram.iter().max().unwrap();

    println!("{:?}", card_histogram);
    println!("{:?} {}", max_val, max_2val);
    return match max_val {
        5 => 7,
        4 => 6,
        3 => {
            if max_2val == 2 {
                5
            } else {
                4
            }
        }
        2 => {
            if max_2val == 2 {
                3
            } else {
                2
            }
        }
        1 => 1,
        _ => {
            println!("huuhhhh");
            0
        }
    }
}

fn main() {
    println!("nanan");
    let filename = "7.in";
    let mut hands: Vec<Hand> = read_to_string(filename).unwrap().lines().map(Hand::from_str).flatten().collect();
    hands.sort_by_key(|h| h.ranking);
    for (i, hand) in hands.iter().enumerate() {
        println!("{}*{}: {:?} {}", i+1, hand.bid, hand.cards, hand.ranking)
    }
    let result: u64 = hands.iter().enumerate().map(|(i, hand)| (i as u64+1) * hand.bid).sum();
    println!("result: {}", result);
}