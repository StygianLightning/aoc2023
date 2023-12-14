use aoc2023::util::{Grid2d, Index2d};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
enum Tile {
    #[default]
    Space,
    Loose,
    Fixed,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            'O' => Self::Loose,
            '#' => Self::Fixed,
            _ => panic!("unknown tile character `{c}`"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/14_training.txt").unwrap();
    println!("{input}");
    let num_lines = input.lines().count();
    let line_length = input.lines().next().unwrap().len();

    let mut grid = Grid2d::new(line_length, num_lines);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            grid[Index2d {
                x: x as i32,
                y: y as i32,
            }] = Tile::from_char(c);
        }
    }

    for y in 0..grid.len_y() {
        for x in 0..grid.len_x() {
            print!(
                "{:?} ",
                grid[Index2d {
                    x: x as i32,
                    y: y as i32
                }]
            );
        }

        println!();
    }
}
