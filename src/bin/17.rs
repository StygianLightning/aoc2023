use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc2023::util::{Direction, Grid2d, Index2d};

fn main() {
    let input_file = if let Some(file) = std::env::args().nth(1) {
        file
    } else {
        "input/17_training.txt".to_owned()
    };
    println!("using input file `{input_file}`");
    let input = std::fs::read_to_string(input_file).unwrap();
    println!("{input}");

    let part2 = if let Some(flag) = std::env::args().nth(2) {
        flag.parse().unwrap()
    } else {
        false
    };

    let mut grid = Grid2d::new(input.lines().next().unwrap().len(), input.lines().count());

    for (y, line) in input.lines().enumerate() {
        for (x, n) in line.char_indices() {
            let num = n.to_string().parse::<u32>().unwrap();
            grid[Index2d {
                x: x as _,
                y: y as _,
            }] = num;
        }
    }

    for y in 0..grid.len_y() as i32 {
        for x in 0..grid.len_x() as i32 {
            print!("{}", grid[Index2d { x, y }]);
        }
        println!();
    }

    // pathfinding

    // we never need to visit the same vertex twice, except if we leave in a new direction.
    // hence we save the direction we visited each vertex in.
    // if we have visited a node from two directions, we never need to revisit it.
    let mut visited = Grid2d::new(grid.len_x(), grid.len_y());
    for y in 0..grid.len_y() as i32 {
        for x in 0..grid.len_x() as i32 {
            visited[Index2d { x, y }] = HashSet::<Direction>::new();
        }
    }

    // For each node we can visit, we remember its position, cost to get there (total heat) as well as the direction we moved in.
    // We cannot move in the same direction twice from the same node.

    // Reverse to make it a min heap.
    let mut queue: BinaryHeap<Reverse<PathfindingNode>> = BinaryHeap::new();

    for direction in [Direction::Left, Direction::Down] {
        queue.push(Reverse(PathfindingNode {
            cost: 0,
            position: Index2d { x: 0, y: 0 },
            previous_position: Index2d { x: 0, y: 0 },
            direction,
        }));
    }

    let goal = Index2d {
        x: grid.len_x() as i32 - 1,
        y: grid.len_y() as i32 - 1,
    };

    let mut goal_cost = None;
    while let Some(node) = queue.pop() {
        let node = node.0;

        // check if goal node.
        if node.position == goal {
            // reached goal
            goal_cost = Some(node.cost);
            break;
        }

        let visited_from_directions = &mut visited[node.position];
        if visited_from_directions.contains(&node.direction) {
            // skip node
            continue;
        } else {
            visited_from_directions.insert(node.direction);
        }

        // visit neighbors.
        for direction in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            if direction == node.direction || direction == node.direction.invert() {
                continue;
            }

            let mut cost = node.cost;
            for i in 0..3 {
                let neighbor = node.position + direction.to_index() * (i + 1);
                if !grid.is_valid(neighbor) {
                    break;
                }
                cost += grid[neighbor];
                queue.push(Reverse(PathfindingNode {
                    cost,
                    position: neighbor,
                    direction,
                    previous_position: node.position,
                }));
            }
        }
    }

    let goal_cost = goal_cost.unwrap();
    println!("cost to reach goal: {goal_cost}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct PathfindingNode {
    cost: u32,
    position: Index2d,
    direction: Direction,
    previous_position: Index2d,
}
