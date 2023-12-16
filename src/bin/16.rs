use std::collections::HashSet;

use aoc2023::util::{Grid2d, Index2d};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Space,
    HorizontalSplit,
    VerticalSplit,
    MirrorRightUp,
    MirrorRightDown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn to_index(self) -> Index2d {
        match self {
            Direction::Up => Index2d { x: 0, y: -1 },
            Direction::Left => Index2d { x: -1, y: 0 },
            Direction::Down => Index2d { x: 0, y: 1 },
            Direction::Right => Index2d { x: 1, y: 0 },
        }
    }
}

fn main() {
    let input_file = if let Some(file) = std::env::args().nth(1) {
        file
    } else {
        "input/16_training.txt".to_owned()
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

    let mut grid = Grid2d::new(input.lines().next().unwrap().len(), input.lines().count());

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            grid[Index2d {
                y: y as _,
                x: x as _,
            }] = match c {
                '.' => Tile::Space,
                '-' => Tile::HorizontalSplit,
                '|' => Tile::VerticalSplit,
                '/' => Tile::MirrorRightUp,
                '\\' => Tile::MirrorRightDown,
                _ => panic!("unknown tile: {c}"),
            };
        }
    }

    println!("reconstructed:");
    print_grid(&grid);

    // follow the lights

    let num_energized_tiles = if part2 {
        (0..grid.len_x())
            .flat_map(|i| {
                [
                    (Index2d { x: i as _, y: 0 }, Direction::Down),
                    (
                        Index2d {
                            x: i as _,
                            y: grid.len_y() as i32 - 1,
                        },
                        Direction::Up,
                    ),
                ]
            })
            .chain((0..grid.len_y()).flat_map(|i| {
                [
                    (Index2d { x: 0, y: i as _ }, Direction::Right),
                    (
                        Index2d {
                            x: grid.len_x() as i32 - 1,
                            y: i as _,
                        },
                        Direction::Left,
                    ),
                ]
            }))
            .map(|(pos, direction)| num_energized_tiles(&grid, pos, direction))
            .max()
            .unwrap()
    } else {
        num_energized_tiles(&grid, Index2d { x: 0, y: 0 }, Direction::Right)
    };
    println!("number of energized tiles: {num_energized_tiles}");
}

fn num_energized_tiles(
    grid: &Grid2d<Tile>,
    start_position: Index2d,
    direction: Direction,
) -> usize {
    // let mut ray_start_positions = vec![(Index2d { x: 0, y: 0 }, Direction::Right)];
    let mut ray_start_positions = vec![(start_position, direction)];
    let mut light_map: Grid2d<HashSet<Direction>> = Grid2d::new(grid.len_x(), grid.len_y());

    for y in 0..grid.len_y() {
        for x in 0..grid.len_x() {
            light_map[Index2d {
                x: x as _,
                y: y as _,
            }] = HashSet::new();
        }
    }

    while let Some((mut position, mut direction)) = ray_start_positions.pop() {
        while grid.is_valid(position) {
            if light_map[position].contains(&direction) {
                break; // we've already seen this
            }
            light_map[position].insert(direction);
            match grid[position] {
                Tile::Space => {}
                Tile::HorizontalSplit => match direction {
                    // tile '-'
                    Direction::Left | Direction::Right => {
                        // noop
                    }
                    Direction::Up | Direction::Down => {
                        // split
                        direction = Direction::Right;
                        ray_start_positions.push((position, Direction::Left));
                    }
                },
                Tile::VerticalSplit => match direction {
                    // tile '|'
                    Direction::Down | Direction::Up => {
                        // noop
                    }
                    Direction::Right | Direction::Left => {
                        // split
                        direction = Direction::Up;
                        ray_start_positions.push((position, Direction::Down));
                    }
                },
                Tile::MirrorRightUp => match direction {
                    // mirror tile '/'
                    Direction::Right => direction = Direction::Up,
                    Direction::Down => direction = Direction::Left,
                    Direction::Up => direction = Direction::Right,
                    Direction::Left => direction = Direction::Down,
                },
                Tile::MirrorRightDown => match direction {
                    // mirror tile '\'
                    Direction::Right => direction = Direction::Down,
                    Direction::Up => direction = Direction::Left,
                    Direction::Down => direction = Direction::Right,
                    Direction::Left => direction = Direction::Up,
                },
            }

            position += direction.to_index();
        }
    }

    let mut num_energized_tiles = 0;
    for x in 0..grid.len_x() {
        for y in 0..grid.len_y() {
            if !light_map[Index2d {
                x: x as _,
                y: y as _,
            }]
            .is_empty()
            {
                num_energized_tiles += 1;
            }
        }
    }

    num_energized_tiles
}

fn print_grid(grid: &Grid2d<Tile>) {
    for y in 0..grid.len_y() {
        let mut line = String::new();
        for x in 0..grid.len_x() {
            match grid[Index2d {
                x: x as _,
                y: y as _,
            }] {
                Tile::Space => line += ".",
                Tile::HorizontalSplit => line += "-",
                Tile::VerticalSplit => line += "|",
                Tile::MirrorRightUp => line += "/",
                Tile::MirrorRightDown => line += "\\",
            }
        }
        println!("{line}");
    }
}
