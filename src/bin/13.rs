use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum MirrorType {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
struct Mirror {
    mirror_type: MirrorType,
    after: usize,
}

#[derive(Debug, Default)]
struct Pattern {
    rows: Vec<String>,
    columns: Vec<String>,
}

impl Pattern {
    pub fn push_line(&mut self, s: &str) {
        self.push_line_owned(s.to_owned())
    }

    pub fn push_line_owned(&mut self, s: String) {
        self.rows.push(s);
    }

    pub fn finalize(&mut self) {
        let len = self.rows[0].len();
        for i in 0..len {
            let mut s = String::with_capacity(len);
            for line in &self.rows {
                s += &line[i..i + 1];
            }
            self.columns.push(s);
        }
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn num_columns(&self) -> usize {
        self.columns.len()
    }

    pub fn find_mirror(&self, part2: bool) -> Option<Mirror> {
        for (strings, mirror, num_strings) in [
            (&self.rows, MirrorType::Horizontal, self.rows.len()),
            (&self.columns, MirrorType::Vertical, self.columns.len()),
        ] {
            if let Some(mirror) = find_mirror_for_strings(mirror, num_strings, part2, strings) {
                return Some(mirror);
            }
        }

        None
    }
}

fn find_mirror_for_strings(
    mirror_type: MirrorType,
    num_strings: usize,
    part2: bool,
    strings: &[String],
) -> Option<Mirror> {
    let mut map = HashMap::new();
    for (i, line) in strings.iter().enumerate() {
        map.entry(line.to_owned()).or_insert(vec![]).push(i);
    }

    let mut inverse_map = HashMap::new();
    for list in map.values() {
        for i in list {
            inverse_map.insert(*i, list[0]);
        }
    }

    let mut mirror_pos = None;

    for i in 0..num_strings - 1 {
        // check if i is a mirror point

        let mut left = i as i32;

        let mut right = i + 1;
        let mut mismatches = vec![];

        while left >= 0 && right < num_strings {
            if inverse_map[&(left as usize)] != inverse_map[&right] {
                // not mirrored
                mismatches.push((left, right));

                if !part2 || mismatches.len() > 1 {
                    // part 1 allows no mismatches, part2 only one
                    break;
                }
            }
            left -= 1;
            right += 1;
        }

        let mirrored = if part2 {
            match &mismatches[..] {
                // for part 2, mirroring requires exactly one smidge:
                // exactly one position must differ for one pair of strings.
                &[(left, right)] => {
                    let a = &strings[left as usize];
                    let b = &strings[right];
                    let num_diff = a.chars().zip(b.chars()).filter(|(a, b)| a != b).count();
                    num_diff == 1
                }
                _ => false,
            }
        } else {
            mismatches.is_empty()
        };

        if mirrored {
            mirror_pos = Some(i);
        }
    }

    mirror_pos.map(|pos| Mirror {
        mirror_type,
        after: pos,
    })
}

fn main() {
    let input = std::fs::read_to_string("input/13_training.txt").unwrap();
    let input = std::fs::read_to_string("input/13.txt").unwrap();

    let part2 = true;

    let mut patterns = vec![];
    patterns.push(Pattern::default());

    for line in input.lines() {
        if line.chars().all(|c| c.is_whitespace()) {
            patterns.last_mut().unwrap().finalize();
            patterns.push(Pattern::default());
        } else {
            patterns.last_mut().unwrap().push_line(line);
        }
    }

    patterns.last_mut().unwrap().finalize();

    let mut total = 0;

    for pattern in &patterns {
        let mirror = pattern.find_mirror(part2).unwrap();
        println!("found mirror: {mirror:?}");

        total += match mirror.mirror_type {
            MirrorType::Vertical => mirror.after + 1,
            MirrorType::Horizontal => (mirror.after + 1) * 100,
        };
    }

    println!("total: {total}");
}
