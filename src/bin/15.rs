fn hash(s: &str) -> u32 {
    let mut result = 0;

    for c in s.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }

    result
}

fn main() {
    let input_file = if let Some(file) = std::env::args().nth(1) {
        file
    } else {
        "input/15_training.txt".to_owned()
    };
    println!("using input file `{input_file}`");
    let mut input = std::fs::read_to_string(input_file).unwrap();
    input.retain(|c| c != '\n');
    println!("{input}");

    let part2 = if let Some(flag) = std::env::args().nth(2) {
        flag.parse().unwrap()
    } else {
        false
    };

    println!("part2 flag: {part2}");

    let mut total = 0;
    for s in input.split(',') {
        total += hash(s);
    }

    println!("total: {total}");
}
