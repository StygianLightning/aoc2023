use std::collections::{hash_map::Entry, HashMap};

use aoc2023::util::{Grid2d, Index2d};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

fn shift_grid(grid: &mut Grid2d<Tile>, start_tiles: &[Index2d], direction: Index2d) {
    for tile in start_tiles {
        let mut first_free_index = *tile;
        let mut current = *tile;

        while grid.is_valid(current) {
            //
            match grid[current] {
                Tile::Space => {}
                Tile::Loose => {
                    if first_free_index != current {
                        grid[first_free_index] = Tile::Loose;
                        grid[current] = Tile::Space;
                    }
                    first_free_index += direction;
                }
                Tile::Fixed => {
                    first_free_index = current + direction;
                }
            }

            current += direction;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]

enum Direction {
    North,
    West,
    South,
    East,
}

fn shift_grid_in_direction(grid: &mut Grid2d<Tile>, direction: Direction) {
    let (start_tiles, direction) = match direction {
        Direction::North => {
            let top_row = (0..grid.len_x())
                .map(|i| Index2d { x: i as i32, y: 0 })
                .collect::<Vec<_>>();
            (top_row, Index2d { x: 0, y: 1 })
        }
        Direction::West => {
            let left_row = (0..grid.len_y())
                .map(|i| Index2d { x: 0, y: i as i32 })
                .collect::<Vec<_>>();
            (left_row, Index2d { x: 1, y: 0 })
        }
        Direction::South => {
            let bottom_row = (0..grid.len_x())
                .map(|i| Index2d {
                    x: i as i32,
                    y: grid.len_y() as i32 - 1,
                })
                .collect::<Vec<_>>();
            (bottom_row, Index2d { x: 0, y: -1 })
        }
        Direction::East => {
            let right_row = (0..grid.len_y())
                .map(|i| Index2d {
                    x: grid.len_x() as i32 - 1,
                    y: i as i32,
                })
                .collect::<Vec<_>>();
            (right_row, Index2d { x: -1, y: 0 })
        }
    };

    shift_grid(grid, &start_tiles, direction);
}

const NUM_CYCLES: usize = 1000000000;

fn main() {
    let input_file = if let Some(file) = std::env::args().nth(1) {
        file
    } else {
        "input/14_training.txt".to_owned()
    };
    println!("using input file `{input_file}`");
    let input = std::fs::read_to_string(input_file).unwrap();
    println!("{input}");

    let part2 = if let Some(flag) = std::env::args().nth(2) {
        flag.parse().unwrap()
    } else {
        false
    };

    println!("part2 flag: {part2}");

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
        let mut dejavu = HashMap::new();
        dejavu.insert(grid.clone(), 0);
        let mut num_iterations = 0;
        let mut loop_start = None;
        for i in 0..NUM_CYCLES {
            shift_grid_in_direction(&mut grid, Direction::North);
            shift_grid_in_direction(&mut grid, Direction::West);
            shift_grid_in_direction(&mut grid, Direction::South);
            shift_grid_in_direction(&mut grid, Direction::East);
            // print_grid(&grid);

            match dejavu.entry(grid.clone()) {
                Entry::Occupied(e) => {
                    println!(
                        "seen before in iteration {}; current iteration: {i}",
                        e.get()
                    );

                    num_iterations = i;
                    loop_start = Some(*e.get());
                    break;
                }
                Entry::Vacant(e) => {
                    e.insert(i);
                }
            }
        }

        if let Some(loop_start) = loop_start {
            let loop_len = num_iterations - loop_start;
            let remaining_iterations = NUM_CYCLES - loop_start - 1;
            let end_index = loop_start + remaining_iterations % loop_len;
            dbg!(loop_start);
            dbg!(num_iterations);
            dbg!(remaining_iterations);
            dbg!(loop_len);
            dbg!(end_index);
            for (k, v) in dejavu {
                if v == end_index {
                    grid = k;
                    break;
                }
            }
        }
    } else {
        shift_grid_in_direction(&mut grid, Direction::North);
        println!("\nshifted:");
        print_grid(&grid);
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

fn print_grid(grid: &Grid2d<Tile>) {
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
}
