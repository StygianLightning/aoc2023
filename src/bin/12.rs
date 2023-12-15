use aoc2023::util::{Grid2d, Index2d};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringStatus {
    Operational,
    // we care about groups of damaged springs
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    status_data: Vec<SpringStatus>,
    damaged_groups: Vec<usize>,
}

impl Record {
    fn from_line(line: &str) -> Self {
        let mut it = line.split_whitespace();
        let status_data = it
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => SpringStatus::Operational,
                '#' => SpringStatus::Damaged,
                '?' => SpringStatus::Unknown,
                _ => panic!("Can't handle spring status {c}"),
            })
            .collect();

        let nums = it.next().unwrap();
        let damaged_groups = nums.split(',').map(|s| s.parse().unwrap()).collect();

        Self {
            status_data,
            damaged_groups,
        }
    }

    fn ways_to_match(&self) -> usize {
        // We don't have to compute or save the resulting spring status vec.
        let mut dp_table = Grid2d::new(self.damaged_groups.len(), self.status_data.len());

        // start with the last group and iterate back to the first
        for group_index in (0..self.damaged_groups.len()).rev() {
            let last_group = group_index == self.damaged_groups.len() - 1;

            let current_group_size = self.damaged_groups[group_index];

            for start_index in 0..self.status_data.len() {
                // check if assigning the group to the blocks starting at start_index can work out: they all have to be unknown or damaged
                let end_index = start_index + current_group_size;

                if end_index > self.status_data.len() {
                    // group would go out of bounds
                    continue;
                }
                if self.status_data[start_index..end_index]
                    .iter()
                    .any(|s| *s == SpringStatus::Operational)
                {
                    // can't assign the group here.
                    continue;
                }

                // check if all the blocks match
                if last_group {
                    // make sure that all remaining blocks are either operational or unknown; if they are, save 1, otherwise 0.
                    // note that since this is the last group, the unknown springs have to be operational
                    if self.status_data[end_index..]
                        .iter()
                        .any(|s| *s == SpringStatus::Damaged)
                    {
                        // nope
                        continue;
                    }
                    dp_table[Index2d {
                        x: group_index as _,
                        y: start_index as _,
                    }] = 1;
                } else {
                    // there needs to be at least one operational (or unknown) spring after every group
                    if end_index >= self.status_data.len()
                        || self.status_data[end_index] == SpringStatus::Damaged
                    {
                        continue;
                    }

                    if group_index == 0 {
                        // We've arrived at the first group.
                        // If there are any damaged springs before the current start index, the current combination is invalid.
                        if self.status_data[0..start_index]
                            .iter()
                            .any(|s| *s == SpringStatus::Damaged)
                        {
                            continue;
                        }
                    }

                    let mut total = 0;
                    let next_group_idx = group_index + 1;
                    let next_group_first_potential_start = end_index + 1;

                    // After one operational spring, we can start assigning the next group.
                    // We already have the solution for that in the dp_table.
                    // Check every option and add them to the dp_table entry for the current group and start index.

                    for next_group_actual_start in
                        next_group_first_potential_start..self.status_data.len()
                    {
                        // we can only use this combination of group assignments if there is no damaged spring between end of the current group and the start of the next one.
                        // otherwise, the resulting combination of groups would have extra damaged springs that would be unaccounted for.
                        if self.status_data
                            [next_group_first_potential_start..next_group_actual_start]
                            .iter()
                            .any(|s| *s == SpringStatus::Damaged)
                        {
                            continue;
                        }

                        total += dp_table[Index2d {
                            x: next_group_idx as _,
                            y: next_group_actual_start as _,
                        }];
                    }
                    dp_table[Index2d {
                        x: group_index as _,
                        y: start_index as _,
                    }] = total;
                }
            }
        }

        let mut total = 0;
        // for the total, check the number of possible assignments for the first group and add them up.

        let first_group_idx = 0;
        for i in 0..self.status_data.len() {
            total += dp_table[Index2d {
                x: first_group_idx,
                y: i as _,
            }];
        }

        total
    }

    fn expand(&mut self) {
        self.damaged_groups = self
            .damaged_groups
            .iter()
            .cloned()
            .cycle()
            .take(self.damaged_groups.len() * 5)
            .collect();
        self.status_data = self
            .status_data
            .iter()
            .cloned()
            .chain(std::iter::once(SpringStatus::Unknown))
            .cycle()
            .take(self.status_data.len() * 5 + 4)
            .collect();
    }
}

fn main() {
    let input = std::fs::read_to_string("input/12.txt").unwrap();
    let part2 = true;
    let mut total = 0;
    for line in input.lines() {
        let mut record = Record::from_line(line);
        if part2 {
            record.expand();
        }
        let ways_to_match = record.ways_to_match();
        total += ways_to_match;
    }

    println!("total ways to match: {total}");
}
