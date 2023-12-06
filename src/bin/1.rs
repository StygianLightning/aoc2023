use std::collections::HashMap;

fn get_first_maybe_encoded_digit(text: &str, map: &HashMap<String, String>) -> String {
    let first = text.chars().enumerate().find(|(_, c)| c.is_numeric());

    if let Some((first_idx, first)) = first {
        if let Some(value) = find_first_number(&text[0..first_idx], map) {
            value
        } else {
            first.to_string()
        }
    } else if let Some(value) = find_first_number(text, map) {
        value
    } else {
        println!("nothing at all in line {text}");
        unreachable!();
    }
}

fn process_line(
    text: &str,
    map: &HashMap<String, String>,
    rev_map: &HashMap<String, String>,
) -> u32 {
    let mut first = get_first_maybe_encoded_digit(text, map);

    let rev_line = text.chars().rev().collect::<String>();
    let last = get_first_maybe_encoded_digit(&rev_line, rev_map);
    first += &last;

    first.parse().unwrap()
}

fn find_first_number(text: &str, map: &HashMap<String, String>) -> Option<String> {
    let mut first_start_idx = text.len();
    let mut value = None;

    for (k, v) in map {
        if text.len() < k.len() {
            continue;
        }
        for i in 0..text.len() - 1 {
            if text[i..].starts_with(k) {
                // potential hit
                if i < first_start_idx {
                    first_start_idx = i;
                    value = Some(v.clone());
                }
            }
        }
    }

    value
}

fn get_spelled_number_mapping() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("one"), String::from("1"));
    map.insert(String::from("two"), String::from("2"));
    map.insert(String::from("three"), String::from("3"));
    map.insert(String::from("four"), String::from("4"));
    map.insert(String::from("five"), String::from("5"));
    map.insert(String::from("six"), String::from("6"));
    map.insert(String::from("seven"), String::from("7"));
    map.insert(String::from("eight"), String::from("8"));
    map.insert(String::from("nine"), String::from("9"));

    map
}

fn reverse_string_map(original_map: &HashMap<String, String>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (k, v) in original_map.iter() {
        let reverse_key = k.chars().rev().collect::<String>();
        map.insert(reverse_key, v.clone());
    }

    map
}

fn main() {
    let map = get_spelled_number_mapping();
    let rev_map = reverse_string_map(&map);
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    //let input = std::fs::read_to_string("input/one_part_2_training.txt").unwrap();
    let total: u32 = input
        .split('\n')
        .map(|l| process_line(l, &map, &rev_map))
        .sum();

    println!("{total}");
}
