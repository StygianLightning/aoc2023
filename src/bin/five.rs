use std::collections::HashMap;

#[derive(Debug, Default)]
struct RangeMap {
    map: HashMap<u32, u32>,
}

impl RangeMap {
    pub fn insert(&mut self, dest: u32, source: u32, len: u32) {
        for i in 0..len {
            self.map.insert(source + i, dest + i);
        }
    }

    pub fn get(&self, val: u32) -> u32 {
        self.map.get(&val).cloned().unwrap_or(val)
    }
}

fn main() {
    let input = std::fs::read_to_string("input/5_training.txt").unwrap();
    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    let seeds = seeds.split(':').nth(1).unwrap();

    let seeds: Vec<u32> = seeds
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    println!("{seeds:?}");
}
