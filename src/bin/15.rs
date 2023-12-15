fn hash(s: &str) -> u32 {
    let mut result = 0;

    for c in s.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }

    result
}

const NUM_BOXES: usize = 256;

#[derive(Debug)]
struct Lense {
    focal_length: u32,
    label: String,
}

#[derive(Debug, Default)]
struct LenseBox {
    lenses: Vec<Lense>,
}

impl LenseBox {
    fn add(&mut self, lense: Lense) {
        if let Some(pos) = self.lenses.iter().position(|l| l.label == lense.label) {
            self.lenses[pos] = lense;
        } else {
            self.lenses.push(lense);
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(pos) = self.lenses.iter().position(|l| l.label == label) {
            self.lenses.remove(pos);
        }
    }
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

    if !part2 {
        let mut total = 0;
        for s in input.split(',') {
            total += hash(s);
        }

        println!("total: {total}");
        return;
    }

    // part 2 logic
    let mut boxes = (0..NUM_BOXES)
        .map(|_| LenseBox::default())
        .collect::<Vec<_>>();

    for s in input.split(',') {
        if s.contains('=') {
            let mut it = s.split('=');
            let label = it.next().unwrap();
            let focal_length = it.next().unwrap().parse::<u32>().unwrap();
            let index = hash(label);
            let lense_box = &mut boxes[index as usize];
            lense_box.add(Lense {
                focal_length,
                label: label.to_owned(),
            })
        } else {
            // remove
            let label = s.split('-').next().unwrap();
            let index = hash(label);
            let lense_box = &mut boxes[index as usize];
            lense_box.remove(label);
        }
    }

    let mut total = 0;
    for (i, b) in boxes.iter().enumerate() {
        let mut lense_power = 0;
        for (lense_idx, lense) in b.lenses.iter().enumerate() {
            let lense_index_multiplier = lense_idx + 1;
            lense_power += lense_index_multiplier as u32 * lense.focal_length;
        }
        let index_multiplier = i as u32 + 1;
        total += lense_power * index_multiplier;
    }

    println!("total: {total}");
}
