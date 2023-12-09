fn main() {
    let input = std::fs::read_to_string("input/9.txt").unwrap();

    let part2 = true;

    let mut pyramids = vec![];
    for sequence in input.lines() {
        let nums = sequence
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        let nums = if part2 {
            nums.rev().collect()
        } else {
            nums.collect()
        };
        let mut pyramid = Pyramid::new(nums);
        pyramid.extend();
        pyramids.push(pyramid);
    }

    let total: i32 = pyramids
        .iter()
        .map(|p| p.nums_per_layer[0].last().unwrap())
        .sum();

    println!("total: {total}");
}

#[derive(Debug)]
struct Pyramid {
    nums_per_layer: Vec<Vec<i32>>,
}

impl Pyramid {
    fn new(nums: Vec<i32>) -> Self {
        let mut nums_per_layer = vec![nums];

        while nums_per_layer.last().unwrap().iter().any(|x| *x != 0) {
            let mut new_layer = vec![];

            let previous_layer = nums_per_layer.last().unwrap();

            for i in 0..previous_layer.len() - 1 {
                new_layer.push(previous_layer[i + 1] - previous_layer[i]);
            }

            if previous_layer.len() < 2 {
                new_layer.push(0);
            }

            nums_per_layer.push(new_layer);
        }

        Self { nums_per_layer }
    }

    fn extend(&mut self) {
        if self.nums_per_layer.len() < 2 {
            return;
        }
        self.nums_per_layer.last_mut().unwrap().push(0);
        for layer_idx in (0..self.nums_per_layer.len() - 1).rev() {
            let add_from_previous_layer = *self.nums_per_layer[layer_idx + 1].last().unwrap();
            let last_in_layer = *self.nums_per_layer[layer_idx].last().unwrap();
            self.nums_per_layer[layer_idx].push(last_in_layer + add_from_previous_layer);
        }
    }
}
