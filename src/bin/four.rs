use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(id: u32, winning_numbers: HashSet<u32>, numbers: Vec<u32>) -> Self {
        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn winning_numbers(&self) -> &HashSet<u32> {
        &self.winning_numbers
    }

    pub fn numbers(&self) -> &[u32] {
        &self.numbers
    }

    pub fn correct_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn from_line(line: &str, id: u32) -> Self {
        let mut numbers = line.split(':');
        let numbers = numbers.nth(1).unwrap();

        let mut numbers = numbers.split('|');
        let winning_numbers = numbers.next().unwrap().trim();
        let numbers = numbers.next().unwrap().trim();

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    pub fn score_part_1(&self) -> u32 {
        let correct_numbers = self.correct_numbers();
        if correct_numbers.is_empty() {
            0
        } else {
            1 << (correct_numbers.len() - 1)
        }
    }

    pub fn update_winning_cards(&self, num_cards: &mut [u32]) {
        for i in 0..self.correct_numbers().len() {
            num_cards[i + 1 + self.id as usize] += num_cards[self.id as usize];
        }
    }
}

fn main() {
    let text = std::fs::read_to_string("input/4.txt").unwrap();
    let cards = text
        .lines()
        .enumerate()
        .map(|(i, l)| Card::from_line(l, i as _))
        .collect::<Vec<_>>();

    let total_winning_sum: u32 = cards.iter().map(Card::score_part_1).sum();
    println!("total winning sum: {total_winning_sum}");

    let mut copies_per_card = vec![1; cards.len()];
    for card in &cards {
        card.update_winning_cards(&mut copies_per_card);
    }
    println!("{copies_per_card:?}");
    let total_num_cards = copies_per_card.iter().sum::<u32>();
    println!("total #cards: {total_num_cards:?}");
}
