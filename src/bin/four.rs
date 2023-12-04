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

    pub fn from_line(line: &str) -> Self {
        let mut numbers = line.split(':');
        let id = numbers.next().unwrap();
        let mut id = id.split_whitespace();
        let id = id.nth(1).unwrap().parse::<u32>().unwrap();

        let numbers = numbers.next().unwrap();

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

    pub fn score(&self) -> u32 {
        let correct_numbers = self.correct_numbers();
        if correct_numbers.is_empty() {
            0
        } else {
            1 << (correct_numbers.len() - 1)
        }
    }
}

fn main() {
    let text = std::fs::read_to_string("input/4_training.txt").unwrap();
    let text = std::fs::read_to_string("input/4.txt").unwrap();
    let cards = text.lines().map(Card::from_line).collect::<Vec<_>>();

    let total_winning_sum: u32 = cards.iter().map(Card::score).sum();
    println!("total winning sum: {total_winning_sum}");

    let mut copies_per_card = vec![1; cards.len()];
}
