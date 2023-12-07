use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/7_training.txt").unwrap();

    let hands_and_bids = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let hand = parts
                .next()
                .unwrap()
                .chars()
                .map(convert_to_card_rank)
                .collect::<Vec<_>>();
            let bid = parts.next().unwrap().parse::<u32>().unwrap();

            analyze_hand(hand, bid)
        })
        .collect::<Vec<_>>();
}

#[derive(Debug)]
struct Hand {
    frequency_map: HashMap<u32, u32>,
    hand: Vec<u32>,
    bid: u32,
}

fn analyze_hand(hand: Vec<u32>, bid: u32) -> Hand {
    let mut frequency_map = HashMap::new();

    for num in &hand {
        let entry = frequency_map.entry(*num).or_default();
        *entry += 1;
    }

    Hand {
        hand,
        frequency_map,
        bid,
    }
}

fn convert_to_card_rank(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_string().parse().unwrap(),
    }
}
