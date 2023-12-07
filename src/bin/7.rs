use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = std::fs::read_to_string("input/7.txt").unwrap();
    // false for pt 1, true for pt 2
    let jacks_are_wildcards = true;

    let mut hands = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let hand = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| convert_to_card_rank(c, jacks_are_wildcards))
                .collect::<Vec<_>>();
            let bid = parts.next().unwrap().parse::<u32>().unwrap();

            analyze_hand(hand, bid, jacks_are_wildcards)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        a.hand_type.cmp(&b.hand_type).then_with(|| {
            for i in 0..5 {
                let a = a.hand[i];
                let b = b.hand[i];
                let ord = a.cmp(&b);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            Ordering::Equal
        })
    });
    for hand in &hands {
        println!("hand type {:?}, hand: {:?}", hand.hand_type, hand.hand)
    }

    let total: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum();

    println!("total: {total}");
}

#[derive(Debug)]
struct Hand {
    hand: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn rank(self) -> u32 {
        match self {
            HandType::FiveOfAKind {} => 6,
            HandType::FourOfAKind {} => 5,
            HandType::FullHouse {} => 4,
            HandType::ThreeOfAKind {} => 3,
            HandType::TwoPair {} => 2,
            HandType::Pair {} => 1,
            HandType::HighCard {} => 0,
        }
    }
}

fn analyze_hand(hand: Vec<u32>, bid: u32, jacks_are_wildcards: bool) -> Hand {
    let mut frequency_map = HashMap::new();

    for num in &hand {
        let entry = frequency_map.entry(*num).or_default();
        *entry += 1;
    }

    struct CardAndFrequency {
        card_rank: u32,
        frequency: u32,
    }

    let mut entry_vec = vec![];
    for (k, v) in &frequency_map {
        entry_vec.push(CardAndFrequency {
            card_rank: *k,
            frequency: *v,
        });
    }

    entry_vec.sort_by(|a, b| {
        a.frequency
            .cmp(&b.frequency)
            .reverse()
            .then(a.card_rank.cmp(&b.card_rank).reverse())
    });

    let mut hand_type = if entry_vec[0].frequency == 5 {
        HandType::FiveOfAKind
    } else if entry_vec[0].frequency == 4 {
        HandType::FourOfAKind
    } else if entry_vec[0].frequency == 3 && entry_vec[1].frequency == 2 {
        HandType::FullHouse
    } else if entry_vec[0].frequency == 3 {
        HandType::ThreeOfAKind
    } else if entry_vec[0].frequency == 2 && entry_vec[1].frequency == 2 {
        HandType::TwoPair
    } else if entry_vec[0].frequency == 2 {
        HandType::Pair
    } else {
        HandType::HighCard
    };

    let jack = convert_to_card_rank('J', jacks_are_wildcards);
    let num_jacks = hand.iter().filter(|c| **c == jack).count() as u32;
    if jacks_are_wildcards && num_jacks > 0 {
        hand_type = match hand_type {
            HandType::FiveOfAKind | HandType::FourOfAKind | HandType::FullHouse => {
                HandType::FiveOfAKind
            }
            HandType::TwoPair => {
                if num_jacks == 1 {
                    HandType::FullHouse
                } else {
                    HandType::FourOfAKind
                }
            }
            HandType::Pair => HandType::ThreeOfAKind,
            HandType::HighCard => HandType::Pair,
            HandType::ThreeOfAKind => HandType::FourOfAKind,
        };
    }

    Hand {
        hand,
        hand_type,
        bid,
    }
}

fn convert_to_card_rank(c: char, jacks_are_wildcards: bool) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if jacks_are_wildcards {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => c.to_string().parse().unwrap(),
    }
}
