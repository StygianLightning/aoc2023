use core::panic;
use std::ops::{Add, Index, IndexMut};

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
    fn to_index<T>(self, grid: &[Vec<T>]) -> Option<(usize, usize)> {
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

    fn from_offsets(a: Offset, b: Offset) -> Self {
        match (a, b) {
            (UP, DOWN) | (DOWN, UP) => Self::UpDown,
            (LEFT, RIGHT) | (RIGHT, LEFT) => Self::LeftRight,
            (UP, RIGHT) | (RIGHT, UP) => Self::UpRight,
            (UP, LEFT) | (LEFT, UP) => Self::UpLeft,
            (DOWN, RIGHT) | (RIGHT, DOWN) => Self::DownRight,
            (DOWN, LEFT) | (LEFT, DOWN) => Self::DownLeft,
            _ => panic!("Can't construct matching tile for offsets {a:?} and {b:?}"),
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

#[derive(Debug)]
struct Loop {
    len: usize,
    loop_grid: Vec<Vec<bool>>,
    start_tile: Tile,
}

fn find_loop(
    grid: &[Vec<Tile>],
    mut current_position: Position,
    mut previous_position: Position,
    goal: Position,
) -> Option<Loop> {
    let mut len = 1;

    let mut loop_grid: Vec<Vec<bool>> = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();

    let previous_index = previous_position.to_index(grid).unwrap();

    let first_non_start_position = current_position;

    loop_grid[previous_index.1][previous_index.0] = true;
    while current_position != goal {
        let index = current_position.to_index(grid).unwrap();
        let tile = grid[index.1][index.0];
        loop_grid[index.1][index.0] = true;

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
            len += 1;
            previous_position = current_position;
            current_position = next_position;

            if current_position == goal {
                let offset_first = Offset {
                    x: first_non_start_position.x - goal.x,
                    y: first_non_start_position.y - goal.y,
                };
                let offset_last = Offset {
                    x: previous_position.x - goal.x,
                    y: previous_position.y - goal.y,
                };

                let start_tile = Tile::from_offsets(offset_first, offset_last);

                println!("start tile: {start_tile:?}");

                return Some(Loop {
                    len,
                    loop_grid,
                    start_tile,
                });
            }
        } else {
            // no connection to previous tile
            break;
        }
    }
    None
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CornerIndex {
    x: i32,
    y: i32,
}

impl CornerIndex {
    fn neighbors(self) -> [CornerIndex; 4] {
        [
            CornerIndex {
                x: self.x - 1,
                y: self.y,
            },
            CornerIndex {
                x: self.x + 1,
                y: self.y,
            },
            CornerIndex {
                x: self.x,
                y: self.y - 1,
            },
            CornerIndex {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug)]
struct CornerGrid {
    connected_to_outside_grid: Vec<Vec<bool>>,
}

impl CornerGrid {
    fn new(grid: &[Vec<bool>]) -> Self {
        Self {
            connected_to_outside_grid: vec![vec![false; grid[0].len() + 1]; grid.len() + 1],
        }
    }

    fn len_x(&self) -> usize {
        self.connected_to_outside_grid[0].len()
    }

    fn len_y(&self) -> usize {
        self.connected_to_outside_grid.len()
    }

    fn is_valid(&self, index: CornerIndex) -> bool {
        index.x >= 0
            && index.y >= 0
            && (index.x as usize) < self.len_x()
            && (index.y as usize) < self.len_y()
    }
}

impl Index<CornerIndex> for CornerGrid {
    type Output = bool;

    fn index(&self, index: CornerIndex) -> &Self::Output {
        &self.connected_to_outside_grid[index.y as usize][index.x as usize]
    }
}

impl IndexMut<CornerIndex> for CornerGrid {
    fn index_mut(&mut self, index: CornerIndex) -> &mut Self::Output {
        &mut self.connected_to_outside_grid[index.y as usize][index.x as usize]
    }
}

fn count_enclosed_tiles(grid: &[Vec<Tile>], loop_grid: &[Vec<bool>]) -> usize {
    let mut corner_grid = CornerGrid::new(loop_grid);

    let len_x = corner_grid.len_x() as i32;
    let len_y = corner_grid.len_y() as i32;

    let mut stack = vec![];
    // the outermost corners are connected to the outside
    for x in 0..len_x {
        let corner_pos = CornerIndex { x, y: 0 };
        stack.push(corner_pos);
        corner_grid[corner_pos] = true;
        let corner_pos = CornerIndex { x, y: len_y - 1 };
        stack.push(corner_pos);
        corner_grid[corner_pos] = true;
    }
    for y in 0..len_y {
        let corner_pos = CornerIndex { x: 0, y };
        corner_grid[corner_pos] = true;
        stack.push(corner_pos);
        let corner_pos = CornerIndex { x: len_x - 1, y };
        corner_grid[corner_pos] = true;
        stack.push(corner_pos);
    }

    // outermost corners are on the stack
    // pop one element at a time, visit all its valid neighbors that aren't connected yet
    // check if we can connect to them (check with tile grid & loop grid)
    // if so, mark the new corner as connected and throw it on the stack
    // in the end, count the number of grid tiles without corners that have been connected.

    while let Some(corner) = stack.pop() {
        for neighbor in corner.neighbors() {
            if !corner_grid.is_valid(neighbor) {
                // outside of corner grid
                continue;
            }
            if corner_grid[neighbor] {
                // already visited
                continue;
            }

            // check if we can visit this one.
            // all outermost corners have already been shortcut by now
            // because their respective elements on the corner grid are initially set to true
            // so upper/lower/right/left actual grid elements are guaranteed to exist

            let blocked = if corner.x == neighbor.x {
                // vertical line, check for horizontal pipe to block the connection

                let y = corner.y.min(neighbor.y);
                let a = Position { x: corner.x - 1, y };
                let b = Position { x: corner.x, y };
                check_tile_for_offset(grid, loop_grid, a, RIGHT)
                    && check_tile_for_offset(grid, loop_grid, b, LEFT)
            } else {
                // horizontal line, check for vertical pipe to block the connection
                assert!(corner.y == neighbor.y);

                let x = corner.x.min(neighbor.x);
                let a = Position { x, y: corner.y - 1 };
                let b = Position { x, y: corner.y };
                check_tile_for_offset(grid, loop_grid, a, DOWN)
                    && check_tile_for_offset(grid, loop_grid, b, UP)
            };
            if !blocked {
                corner_grid[neighbor] = true;
                stack.push(neighbor);
            }
        }
    }

    let mut num_enclosed = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, _val) in row.iter().enumerate() {
            if loop_grid[r][c] {
                // continue;
            }

            let grid_corners = [
                CornerIndex {
                    x: c as i32,
                    y: r as i32,
                },
                CornerIndex {
                    x: c as i32 + 1,
                    y: r as i32,
                },
                CornerIndex {
                    x: c as i32,
                    y: r as i32 + 1,
                },
                CornerIndex {
                    x: c as i32 + 1,
                    y: r as i32 + 1,
                },
            ];

            if grid_corners.into_iter().all(|c| !corner_grid[c]) {
                num_enclosed += 1;
            }
        }
    }

    num_enclosed
}

fn check_tile_for_offset(
    grid: &[Vec<Tile>],
    loop_grid: &[Vec<bool>],
    position: Position,
    offset: Offset,
) -> bool {
    let index = position.to_index(grid).unwrap();
    let loop_tile = loop_grid[index.1][index.0];
    let pipe_tile = grid[index.1][index.0];
    let contains_offset = pipe_tile.neighbors().contains(&offset);
    loop_tile && contains_offset
}

fn main() {
    let input = std::fs::read_to_string("input/10_training_2_c.txt").unwrap();
    // let input = std::fs::read_to_string("input/10_training_mine.txt").unwrap();
    // let input = std::fs::read_to_string("input/10_training_2_b.txt").unwrap();
    // let input = std::fs::read_to_string("input/10_training_foo.txt").unwrap();
    let input = std::fs::read_to_string("input/10.txt").unwrap();
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(char_to_tile).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = Position { x: -1, y: -1 };
    for (y, row) in grid.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == Tile::Start {
                start = Position {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }

    println!("start position: {start:?}");

    let start_neighbors = [start + UP, start + DOWN, start + RIGHT, start + LEFT];

    for potential_loop_start in start_neighbors {
        if potential_loop_start.to_index(&grid).is_some() {
            if let Some(found_loop) = find_loop(&grid, potential_loop_start, start, start) {
                println!("max distance: {}", found_loop.len / 2);

                grid[start.y as usize][start.x as usize] = found_loop.start_tile;
                let num_enclosed = count_enclosed_tiles(&grid, &found_loop.loop_grid);

                println!("num enclosed tiles: {num_enclosed}");
                return;
            }
        }
    }
}
