#[derive(Debug, Clone, Copy)]
enum SpringStatus {
    Operational,
    // we care about groups of damaged springs
    DamagedGroup(usize),
    Unknown,
}

#[derive(Debug, Default)]
struct Record {
    status: Vec<SpringStatus>,
    damaged_groups: Vec<usize>,
}

impl Record {
    fn from_line(line: &str) -> Self {
        let mut ret = Self::default();

        let mut it = line.split_whitespace();
        let status_data = it.next().unwrap();
        let nums = it.next().unwrap();

        for c in status_data.chars() {
            match c {
                '#' => {
                    let status = ret
                        .status
                        .last()
                        .cloned()
                        .unwrap_or(SpringStatus::Operational);
                    if let SpringStatus::DamagedGroup(n) = status {
                        // extend group size by one.
                        *ret.status.last_mut().unwrap() = SpringStatus::DamagedGroup(n + 1);
                    } else {
                        // new damaged group of size at least 1.
                        ret.status.push(SpringStatus::DamagedGroup(1));
                    }
                }
                '.' => match ret.status.last().cloned() {
                    None | Some(SpringStatus::Operational) => {}
                    _ => ret.status.push(SpringStatus::Operational),
                },
                '?' => ret.status.push(SpringStatus::Unknown),
                _ => panic!("unknown status: {c}"),
            }
        }

        for num in nums.split(',') {
            ret.damaged_groups.push(num.parse().unwrap());
        }

        ret
    }
}

fn main() {
    let input = std::fs::read_to_string("input/12_training.txt").unwrap();
    for line in input.lines() {
        let record = Record::from_line(line);

        println!("{record:?}");
    }
}
