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

    let mut product = 1;
    for (time, target) in race_data.iter().cloned() {
        // could binary search this
        if let Some(first_greater) = (0..time).find(|i| i * (time - i) > target) {
            let last_greater = time - first_greater;
            let ways_to_win = last_greater - first_greater + 1;
            product *= ways_to_win;
        }
    }

    println!("ways to win product: {product}");
}
