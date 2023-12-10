use core::panic;

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
            || self.x as usize >= grid.len()
            || self.y < 0
            || self.y as usize >= grid[0].len()
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

const UP: Offset = Offset { x: 0, y: -1 };
const DOWN: Offset = Offset { x: 0, y: 1 };
const LEFT: Offset = Offset { x: -1, y: 0 };
const RIGHT: Offset = Offset { x: 1, y: 0 };

impl Tile {
    fn neighbors(self) -> (Offset, Offset) {
        match self {
            Tile::UpDown => (UP, DOWN),
            Tile::LeftRight => (LEFT, RIGHT),
            Tile::UpRight => (UP, RIGHT),
            Tile::UpLeft => (UP, LEFT),
            Tile::DownRight => (DOWN, RIGHT),
            Tile::DownLeft => (DOWN, LEFT),
            Tile::Ground => (Offset::default(), Offset::default()),
            Tile::Start => (Offset::default(), Offset::default()),
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
}
