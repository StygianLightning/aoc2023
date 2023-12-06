fn main() {
    let input = std::fs::read_to_string("input/6_training.txt").unwrap();
    let mut lines = input.lines();
    let time_list = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap());
    let distance_list = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    let race_data = time_list.zip(distance_list).collect::<Vec<_>>();

    println!("race data: {race_data:?}");
}
