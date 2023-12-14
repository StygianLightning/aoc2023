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
    let input_file = if let Some(file) = std::env::args().nth(1) {
        file
    } else {
        "input/14_training.txt".to_owned()
    };
    println!("using input file `{input_file}`");
    let input = std::fs::read_to_string(input_file).unwrap();
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

    let mut total = 0;
    for x in 0..grid.len_x() as i32 {
        let mut load_next = grid.len_y() as i32; // the examples are 1-indexed
        let mut load_this_column = 0;
        for y in 0..grid.len_y() as i32 {
            //
            let idx = Index2d { x, y };
            match grid[idx] {
                Tile::Space => continue,
                Tile::Loose => {
                    load_this_column += load_next;
                    load_next -= 1;
                }
                Tile::Fixed => {
                    load_next = grid.len_y() as i32 - y - 1;
                }
            }
        }

        println!("load column {x}: {load_this_column}");
        total += load_this_column;
    }

    println!("total load: {total}");
}
