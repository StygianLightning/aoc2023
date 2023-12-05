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

const NUM_MAPS: u32 = 7;

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

    let mut map_vec = vec![];

    let mut skip = 2;
    for i in 0..NUM_MAPS {
        let maps = lines.clone().skip(skip);
        let seed_to_soil_map = maps.clone().take_while(|l| l.starts_with(char::is_numeric));
        println!("map {i}");
        let mut range_map = RangeMap::default();
        for line in seed_to_soil_map {
            skip += 1;
            println!("{line}");
            let mut nums = line
                .split_ascii_whitespace()
                .take(3)
                .map(|s| s.parse::<u32>().unwrap());
            let dest = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();
            range_map.insert(dest, src, len);
        }
        println!();

        map_vec.push(range_map);
        skip += 2; // skip new line and next map declaration
    }

    let closest_converted = seeds
        .iter()
        .map(|seed| convert(*seed, &map_vec))
        .min()
        .unwrap();

    println!("closest location: {closest_converted}");
}

fn convert(num: u32, maps: &Vec<RangeMap>) -> u32 {
    let mut ret = num;
    for map in maps {
        ret = map.get(ret);
    }
    ret
}
