use std::ops::Range;

#[derive(Debug, Default)]
struct RangeMap {
    range_mappings: Vec<RangeMapping>,
}

#[derive(Debug, Default, Clone, Copy)]
struct RangeMapping {
    source: u64,
    dest: u64,
    len: u64,
}

impl RangeMapping {
    fn get(&self, val: u64) -> Option<u64> {
        if val >= self.source && val < self.source + self.len {
            Some(self.dest + (val - self.source))
        } else {
            None
        }
    }

    fn source_end(&self) -> u64 {
        self.source + self.len
    }

    fn dest_end(&self) -> u64 {
        self.source + self.len
    }
}

impl RangeMap {
    pub fn insert(&mut self, dest: u64, source: u64, len: u64) {
        self.range_mappings.push(RangeMapping { source, dest, len });
    }

    pub fn sort(&mut self) {
        self.range_mappings.sort_by(|a, b| a.source.cmp(&b.source));
    }

    pub fn get(&self, val: u64) -> u64 {
        for mapping in &self.range_mappings {
            if let Some(mapped_value) = mapping.get(val) {
                return mapped_value;
            }
        }
        val
    }

    fn get_mapping(&self, range: Range<u64>) -> Option<RangeMapping> {
        // could binary search here
        for mapping in &self.range_mappings {
            if let Some(_mapped_value) = mapping.get(range.start) {
                return Some(*mapping);
            }
        }
        None
    }
}

const NUM_MAPS: u64 = 7;

fn main() {
    let input = std::fs::read_to_string("input/5_training.txt").unwrap();
    let input = std::fs::read_to_string("input/5.txt").unwrap();
    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    let seeds = seeds.split(':').nth(1).unwrap();

    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut seed_ranges = vec![];

    for i in 0..seeds.len() / 2 {
        let start = seeds[i * 2];
        let len = seeds[i * 2 + 1];
        seed_ranges.push(start..start + len);
    }

    println!("{seed_ranges:?}");

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
                .map(|s| s.parse::<u64>().unwrap());
            let dest = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();
            range_map.insert(dest, src, len);
        }
        println!();

        range_map.sort();

        map_vec.push(range_map);
        skip += 2; // skip new line and next map declaration
    }

    let closest_converted = seeds
        .iter()
        .map(|seed| convert(*seed, &map_vec))
        .min()
        .unwrap();

    println!("closest location part 1: {closest_converted}");

    let mut range_stack = seed_ranges;

    for map in &map_vec {
        range_stack = lookup_ranges(range_stack, map);
    }

    let min_range = range_stack.iter().min_by_key(|r| r.start).unwrap();
    println!("min location for all seed ranges: {}", min_range.start);
}

fn convert(num: u64, maps: &Vec<RangeMap>) -> u64 {
    let mut ret = num;
    for map in maps {
        ret = map.get(ret);
    }
    ret
}

fn lookup_ranges(mut ranges_stack: Vec<Range<u64>>, maps: &RangeMap) -> Vec<Range<u64>> {
    let mut output_stack = vec![];

    while let Some(range) = ranges_stack.pop() {
        if let Some(first_overlapping_mapping) = maps.get_mapping(range.clone()) {
            let mapping_end_element = first_overlapping_mapping.source_end();

            if mapping_end_element < range.end {
                // put in the completed part
                let mapped_start = first_overlapping_mapping.get(range.start).unwrap();
                let len = mapping_end_element - range.start;
                output_stack.push(mapped_start..mapped_start + len);

                // mapping was not completed, take care of the rest in a later loop
                let remainder_start = mapping_end_element;
                let remainder_len = (range.end - range.start) - len;
                ranges_stack.push(remainder_start..remainder_start + remainder_len);
            } else {
                // fully mapped
                let mapped_start = first_overlapping_mapping.get(range.start).unwrap();

                let len = range.end - range.start;
                output_stack.push(mapped_start..mapped_start + len);
            }
        } else {
            // completely free range
            output_stack.push(range);
        }
    }

    output_stack
}
