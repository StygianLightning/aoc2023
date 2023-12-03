use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct NumberRange {
    pub line: usize,
    pub start: usize,
    pub len: usize,
    pub number: u32,
}

fn extract_number_range_from_line(line: &str, lineidx: usize) -> Vec<NumberRange> {
    let mut ret = vec![];

    let mut next_unseen_index = 0;

    while next_unseen_index < line.len() {
        let remaining_line = &line[next_unseen_index..];
        let after_non_numeric = remaining_line
            .char_indices()
            .skip_while(|(_, c)| !c.is_numeric());
        let mut num = after_non_numeric.take_while(|(_i, c)| c.is_numeric());

        if let Some((number_start, c)) = num.next() {
            let number_last = if let Some((last, _)) = num.last() {
                last
            } else {
                number_start
            };

            let len = number_last - number_start + 1;

            let number = remaining_line[number_start..=number_last]
                .parse::<u32>()
                .unwrap();

            ret.push(NumberRange {
                line: lineidx,
                start: number_start + next_unseen_index,
                len,
                number,
            });

            next_unseen_index += number_last + 1;
        } else {
            break;
        }
    }

    ret
}

#[derive(Default, Debug)]
struct ValidityMatrix {
    map: HashMap<(usize, usize), bool>,
}

impl ValidityMatrix {
    pub fn valid(&self, (line, idx): (usize, usize)) -> bool {
        *self.map.get(&(line, idx)).unwrap_or(&false)
    }

    pub fn set_valid(&mut self, (line, idx): (usize, usize)) {
        self.map.insert((line, idx), true);
    }
}

static NEIGHBORS: &[(i32, i32)] = &[
    // previous line
    (-1, -1),
    (-1, 0),
    (-1, 1),
    // same line
    (0, -1),
    (0, 0),
    (0, 1),
    // next line
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_neighbor_idx(
    start: (usize, usize),
    offset: (i32, i32),
    num_lines: usize,
    line_length: usize,
) -> Option<(usize, usize)> {
    let (x, y) = (start.0 as i32, start.1 as i32);
    let (x, y) = (x + offset.0, y + offset.1);

    if x < 0 || y < 0 || x as usize >= num_lines || y as usize >= line_length {
        None
    } else {
        Some((x as usize, y as usize))
    }
}

fn extract_validity_neighboring_matrix(text: &str) -> ValidityMatrix {
    let mut ret = ValidityMatrix::default();

    let num_lines = text.lines().count();
    let line_len = text.lines().next().unwrap().len();

    for (line_idx, line) in text.lines().enumerate() {
        for (idx_in_line, c) in line.char_indices() {
            if !c.is_numeric() && c != '.' {
                // symbol

                // for each neighbor, set it to valid.
                for offset in NEIGHBORS.iter() {
                    if let Some(pos) =
                        get_neighbor_idx((line_idx, idx_in_line), *offset, num_lines, line_len)
                    {
                        ret.set_valid(pos);
                    }
                }
            }
        }
    }

    ret
}

fn is_valid(range: NumberRange, matrix: &ValidityMatrix) -> bool {
    (range.start..range.start + range.len).any(|i| matrix.valid((range.line, i)))
}

fn main() {
    // let input = std::fs::read_to_string("input/3_training.txt").unwrap();
    // let input = std::fs::read_to_string("input/3_mine.txt").unwrap();
    let input = std::fs::read_to_string("input/3.txt").unwrap();

    let mut number_ranges: Vec<NumberRange> = vec![];

    for (line_idx, line) in input.lines().enumerate() {
        let ranges = extract_number_range_from_line(line, line_idx);
        number_ranges.extend(ranges);
    }

    let validity_matrix = extract_validity_neighboring_matrix(&input);

    let valid_number_ranges = number_ranges
        .iter()
        .filter(|r| is_valid(**r, &validity_matrix))
        .collect::<Vec<_>>();

    let valid_range_number_sum: u32 = valid_number_ranges.iter().map(|r| r.number).sum();
    println!("total part number sum: {valid_range_number_sum}");
}
