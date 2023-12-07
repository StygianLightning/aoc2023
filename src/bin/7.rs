use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = std::fs::read_to_string("input/7_training.txt").unwrap();

    let mut hands = input
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

    hands.sort_by(|a, b| {
        hand_type_rank(a.hand_type)
            .cmp(&hand_type_rank(b.hand_type))
            .then_with(|| {
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
    println!("sorted hands: {hands:#?}");
}

#[derive(Debug)]
struct Hand {
    hand: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

#[derive(Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind {
        card_rank: u32,
    },
    FourOfAKind {
        card_rank: u32,
    },
    FullHouse {
        triple_card_rank: u32,
        double_card_rank: u32,
    },
    ThreeOfAKind {
        card_rank: u32,
    },
    TwoPair {
        higher: u32,
        lower: u32,
    },
    Pair {
        card_rank: u32,
    },
    HighCard {
        card_rank: u32,
    },
}

fn hand_type_rank(hand_type: HandType) -> u32 {
    match hand_type {
        HandType::FiveOfAKind { .. } => 7,
        HandType::FourOfAKind { .. } => 6,
        HandType::FullHouse { .. } => 5,
        HandType::ThreeOfAKind { .. } => 4,
        HandType::TwoPair { .. } => 3,
        HandType::Pair { .. } => 2,
        HandType::HighCard { .. } => 1,
    }
}

fn analyze_hand(hand: Vec<u32>, bid: u32) -> Hand {
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

    let hand_type = if entry_vec[0].frequency == 5 {
        HandType::FiveOfAKind {
            card_rank: entry_vec[0].card_rank,
        }
    } else if entry_vec[0].frequency == 4 {
        HandType::FourOfAKind {
            card_rank: entry_vec[0].card_rank,
        }
    } else if entry_vec[0].frequency == 3 && entry_vec[1].frequency == 2 {
        HandType::FullHouse {
            triple_card_rank: entry_vec[0].card_rank,
            double_card_rank: entry_vec[1].card_rank,
        }
    } else if entry_vec[0].frequency == 3 {
        HandType::ThreeOfAKind {
            card_rank: entry_vec[0].card_rank,
        }
    } else if entry_vec[0].frequency == 2 && entry_vec[1].frequency == 2 {
        HandType::TwoPair {
            higher: entry_vec[0].card_rank,
            lower: entry_vec[1].card_rank,
        }
    } else if entry_vec[0].frequency == 2 {
        HandType::Pair {
            card_rank: entry_vec[0].card_rank,
        }
    } else {
        HandType::HighCard {
            card_rank: entry_vec[0].card_rank,
        }
    };

    Hand {
        hand,
        hand_type,
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
