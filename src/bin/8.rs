use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Node {
    index: usize,
    left: usize,
    right: usize,
}

#[derive(Debug, Default)]
struct Network {
    nodes: Vec<Node>,

    name_to_id: HashMap<String, usize>,
}

impl Network {
    fn get_or_insert(&mut self, name: &str) -> usize {
        let id = *self
            .name_to_id
            .entry(name.to_owned())
            .or_insert(self.nodes.len());

        if id >= self.nodes.len() {
            self.nodes.push(Node {
                index: id,
                left: 0,
                right: 0,
            })
        }

        id
    }

    fn insert_edges(&mut self, name: &str, left: &str, right: &str) {
        let start = self.get_or_insert(name);
        let left = self.get_or_insert(left);
        let right = self.get_or_insert(right);

        self.nodes[start].left = left;
        self.nodes[start].right = right;
    }

    fn node_neighbor(&self, node: usize, direction: Direction) -> usize {
        let node = &self.nodes[node];
        match direction {
            Direction::Left => node.left,
            Direction::Right => node.right,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/8.txt").unwrap();

    let multiple_start_and_target_nodes = true;

    let mut lines = input.lines();
    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Direction::try_from(c).unwrap())
        .collect();

    let mut network = Network::default();
    for line in lines.skip(1) {
        let mut node_and_neighbors = line.split('=');
        let node = node_and_neighbors.next().unwrap().trim();
        let rest = node_and_neighbors.next().unwrap().trim();
        let mut left_right = rest.split('(').nth(1).unwrap().split(',');
        let left = left_right.next().unwrap().trim();
        let right = left_right.next().unwrap().split(')').next().unwrap().trim();

        network.insert_edges(node, left, right);
    }

    let start_node_filter = if multiple_start_and_target_nodes {
        "A"
    } else {
        "AAA"
    };
    let target_node_filter = if multiple_start_and_target_nodes {
        "Z"
    } else {
        "ZZZ"
    };

    let mut current_nodes = network
        .name_to_id
        .iter()
        .filter(|(k, _v)| k.ends_with(start_node_filter))
        .map(|(_, v)| *v)
        .collect::<Vec<_>>();

    let target_nodes = network
        .name_to_id
        .iter()
        .filter(|(k, _v)| k.ends_with(target_node_filter))
        .map(|(_, v)| *v)
        .collect::<HashSet<_>>();

    // follow instructions
    let mut distance = 0;
    for (iteration, direction) in directions.iter().cycle().cloned().enumerate() {
        if current_nodes.iter().all(|n| target_nodes.contains(n)) {
            distance = iteration;
            break;
        }

        for node in &mut current_nodes {
            let new_node = network.node_neighbor(*node, direction);
            *node = new_node;
        }
    }

    println!("distance: {distance}");
}
