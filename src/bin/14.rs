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

    fn to_char(self) -> char {
        match self {
            Tile::Space => '.',
            Tile::Loose => 'O',
            Tile::Fixed => '#',
        }
    }
}

fn shift_grid_north(grid: &mut Grid2d<Tile>) {
    for x in 0..grid.len_x() as i32 {
        let mut first_free_from_top = 0;
        for y in 0..grid.len_y() as i32 {
            //
            let idx = Index2d { x, y };
            match grid[idx] {
                Tile::Space => continue,
                Tile::Loose => {
                    if first_free_from_top < y {
                        grid[Index2d {
                            x,
                            y: first_free_from_top,
                        }] = Tile::Loose;
                        grid[idx] = Tile::Space;
                    }
                    first_free_from_top += 1;
                }
                Tile::Fixed => {
                    first_free_from_top = y + 1;
                }
            }
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

    let part2 = false;

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

    if part2 {
    } else {
        shift_grid_north(&mut grid);
    }

    println!("\nshifted:");
    for y in 0..grid.len_y() {
        let mut s = String::new();
        for x in 0..grid.len_x() {
            s += &grid[Index2d {
                x: x as i32,
                y: y as i32,
            }]
            .to_char()
            .to_string();
        }

        println!("{s}");
    }

    let mut total = 0;
    for y in 0..grid.len_y() as i32 {
        for x in 0..grid.len_x() as i32 {
            if grid[Index2d { x, y }] == Tile::Loose {
                total += grid.len_y() - (y as usize);
            }
        }
    }

    println!("total load: {total}");
}
