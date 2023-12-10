use core::panic;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Ground,
    Start,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn to_index<T>(self, grid: &Vec<Vec<T>>) -> Option<(usize, usize)> {
        if self.x < 0
            || self.x as usize >= grid[0].len()
            || self.y < 0
            || self.y as usize >= grid.len()
        {
            None
        } else {
            Some((self.x as usize, self.y as usize))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct Offset {
    x: i32,
    y: i32,
}

impl Offset {
    fn is_valid(self) -> bool {
        self.x != 0 || self.y != 0
    }
}

impl Add<Offset> for Position {
    type Output = Position;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
const UP: Offset = Offset { x: 0, y: -1 };
const DOWN: Offset = Offset { x: 0, y: 1 };
const LEFT: Offset = Offset { x: -1, y: 0 };
const RIGHT: Offset = Offset { x: 1, y: 0 };

impl Tile {
    fn neighbors(self) -> [Offset; 2] {
        match self {
            Tile::UpDown => [UP, DOWN],
            Tile::LeftRight => [LEFT, RIGHT],
            Tile::UpRight => [UP, RIGHT],
            Tile::UpLeft => [UP, LEFT],
            Tile::DownRight => [DOWN, RIGHT],
            Tile::DownLeft => [DOWN, LEFT],
            Tile::Ground => [Offset::default(), Offset::default()],
            Tile::Start => [Offset::default(), Offset::default()],
        }
    }
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '|' => Tile::UpDown,
        '-' => Tile::LeftRight,
        'L' => Tile::UpRight,
        'J' => Tile::UpLeft,
        '7' => Tile::DownLeft,
        'F' => Tile::DownRight,
        '.' => Tile::Ground,
        'S' => Tile::Start,
        _ => panic!("unknown tile: {c}"),
    }
}

fn main() {
    let input = std::fs::read_to_string("input/10_training.txt").unwrap();
    let grid = input
        .lines()
        .map(|line| line.chars().map(char_to_tile).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{grid:#?}");

    let mut start = Position { x: -1, y: -1 };
    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == Tile::Start {
                start = Position {
                    x: j as i32,
                    y: i as i32,
                };
            }
        }
    }

    println!("start position: {start:?}");

    let start_neighbors = [start + UP, start + DOWN, start + RIGHT, start + LEFT];

    for potential_loop_start in start_neighbors {
        if potential_loop_start.to_index(&grid).is_some() {
            if let Some(found_loop) = find_loop(&grid, potential_loop_start, start, start) {
                println!("found loop for start position {potential_loop_start:?}: {found_loop:?}");
                println!("max distance: {}", found_loop.len / 2);
            }
        }
    }
}

#[derive(Debug)]
struct Loop {
    len: usize,
}

fn find_loop(
    grid: &Vec<Vec<Tile>>,
    mut current_position: Position,
    mut previous_position: Position,
    goal: Position,
) -> Option<Loop> {
    let mut len = 1;
    while current_position != goal {
        let index = current_position.to_index(grid).unwrap();
        let tile = grid[index.1][index.0];

        let mut next_position = Position { x: -1, y: -1 };
        let mut connected = false;
        for offset in tile.neighbors() {
            if !offset.is_valid() {
                // e.g. ground tile -> no loop
                break;
            }

            let neighbor_pos = current_position + offset;

            if neighbor_pos.to_index(grid).is_none() {
                // off grid -> no loop
                break;
            }

            if neighbor_pos == previous_position {
                connected = true;
            } else {
                next_position = neighbor_pos;
            }
        }

        if connected {
            previous_position = current_position;
            current_position = next_position;
            len += 1;

            if current_position == goal {
                return Some(Loop { len });
            }
        } else {
            // no connection to previous tile
            break;
        }
    }
    None
}
