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
    name: String,
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
                name: name.to_owned(),
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

    let mut start_nodes = network
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

    let mut history_per_node = vec![];

    // follow instructions for each node separately until a loop is reached
    for start_node in &start_nodes {
        let mut node_history = NodeHistory::default();

        let mut current_node = *start_node;
        let mut visited_relative_nodes = HashSet::new();

        let mut last_seen_node = IterationNode {
            node: current_node,
            relative_iteration: 0,
        };
        visited_relative_nodes.insert(last_seen_node);

        for (iteration, direction) in directions.iter().cycle().cloned().enumerate() {
            current_node = network.node_neighbor(current_node, direction);
            let iteration = (iteration + 1) % directions.len();

            last_seen_node = IterationNode {
                node: current_node,
                relative_iteration: iteration,
            };

            if visited_relative_nodes.contains(&last_seen_node) {
                break;
            }
            visited_relative_nodes.insert(last_seen_node);
            node_history.iteration_nodes.push(last_seen_node);
        }

        let loop_start = node_history
            .iteration_nodes
            .iter()
            .position(|n| *n == last_seen_node)
            .unwrap();

        let loop_info = LoopInfo {
            start: loop_start,
            length: node_history.iteration_nodes.len() - loop_start,
        };

        node_history.loop_info = loop_info;

        history_per_node.push(node_history);
    }

    for node_history in &history_per_node {
        let node_name = &network.nodes[node_history.iteration_nodes[0].node].name;
        println!(
            "loop info for node {node_name}: {:#?}",
            node_history.loop_info
        );
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct IterationNode {
    node: usize,
    relative_iteration: usize,
}

#[derive(Debug, Default)]
struct NodeHistory {
    iteration_nodes: Vec<IterationNode>,
    loop_info: LoopInfo,
}

#[derive(Debug, Clone, Copy, Default)]
struct LoopInfo {
    start: usize,
    length: usize,
}
