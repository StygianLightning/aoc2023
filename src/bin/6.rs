fn main() {
    let input = std::fs::read_to_string("input/6_training.txt").unwrap();
    let input = std::fs::read_to_string("input/6.txt").unwrap();
    let mut lines = input.lines();
    let time_list = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());
    let distance_list = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    let race_data = time_list.zip(distance_list).collect::<Vec<_>>();

    println!("race data: {race_data:?}");

    let product = find_ways_to_win_product(&race_data);

    println!("part 1 ways to win product: {product}");

    let (time_str, dist_str) = race_data
        .iter()
        .map(|(time, dist)| (format!("{time}"), format!("{dist}")))
        .reduce(|(time_a, dist_a), (time_b, dist_b)| (time_a + &time_b, dist_a + &dist_b))
        .unwrap();

    let single_race_data = vec![(
        time_str.parse::<u64>().unwrap(),
        dist_str.parse::<u64>().unwrap(),
    )];

    println!("single race data (pt 2): {single_race_data:?}");

    let product_part_2 = find_ways_to_win_product(&single_race_data);
    println!("part 2 ways to win product: {product_part_2}");
}

fn find_ways_to_win_product(race_data: &[(u64, u64)]) -> u64 {
    let mut product = 1;
    for (time, target) in race_data.iter().cloned() {
        // could binary search this
        if let Some(first_greater) = (0..time).find(|i| i * (time - i) > target) {
            let last_greater = time - first_greater;
            let ways_to_win = last_greater - first_greater + 1;
            product *= ways_to_win;
        }
    }

    product
}
