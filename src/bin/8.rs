use std::collections::{HashMap, HashSet};

use gcd::Gcd;

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

    fn node_name(&self, node: usize) -> &str {
        &self.nodes[node].name
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
fn least_common_multiple(x: u128, y: u128) -> u128 {
    x * y / x.gcd(y)
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

    let start_nodes = network
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
        node_history.iteration_nodes.push(last_seen_node);

        for (iteration, direction) in directions.iter().cycle().cloned().enumerate() {
            let previous_node = current_node % directions.len();
            let previous_iteration = iteration;
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

        let loop_len_mod_direction_len = node_history.loop_info.length % directions.len();
        let loop_len_div_direction_len = node_history.loop_info.length / directions.len();

        println!("loop length % direction length: {loop_len_mod_direction_len}");
        println!("loop length / direction length: {loop_len_div_direction_len}");
    }

    let target_nodes_per_start_node = history_per_node
        .iter()
        .map(|history| {
            history
                .iteration_nodes
                .iter()
                .enumerate()
                .filter_map(|(i, n)| target_nodes.contains(&n.node).then_some(i))
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    for (idx, target_node_ids) in target_nodes_per_start_node.iter().enumerate() {
        println!("target_nodes for start node index {idx}: {target_node_ids:?}");
        println!(
            "first target_node for start node index {idx} % directions.len(): {:} / directions.len(): {}",
            target_node_ids.iter().next().unwrap() % directions.len(),
            target_node_ids.iter().next().unwrap() / directions.len()
        );
    }

    // Turns out there's actually only one target index in each node history, and that target is always on the loop in our input files.
    // Note that in general, it's possible that there are multiple target indices that need to be taken into account
    let target_indices = target_nodes_per_start_node
        .iter()
        .map(|coll| coll.iter().next().unwrap())
        .cloned()
        .collect::<Vec<_>>();

    // start index: largest individual index. These are actually all on multiples of directions.len()
    let largest_target_idx = *target_indices.iter().max().unwrap();
    println!("largest target idx: {}", largest_target_idx);

    //now need least common multiple of loop lengths to make sure all end up on their target at the same time
    let loop_lengths_direction_multiples = history_per_node
        .iter()
        .map(|h| (h.loop_info.length / directions.len()) as u128)
        .collect::<Vec<_>>();

    println!(
        "loop lengths as multiples of directions.len(): {:?}",
        loop_lengths_direction_multiples
    );

    let least_common_multiple_loop_len = loop_lengths_direction_multiples
        .iter()
        .cloned()
        .reduce(least_common_multiple)
        .unwrap();

    println!("least common multiple loop len: {least_common_multiple_loop_len}");

    // Normally, this wouldn't be enough -- the nodes could be offset and require more iterations.
    // However, all target nodes in the input files are at the end of the loops, so we can cheat a little:
    let total = least_common_multiple_loop_len * directions.len() as u128;
    println!("total: {total}");
}

#[cfg(test)]
mod test {
    use super::*;
    use gcd::Gcd;

    #[test]
    fn test_gdc() {
        assert_eq!(2u32.gcd(4), 2);
        assert_eq!(2u32.gcd(5), 1);
        assert_eq!(13u32.gcd(5), 1);
        assert_eq!(32u32.gcd(12), 4);
    }

    #[test]
    fn test_least_common_multiple() {
        assert_eq!(least_common_multiple(13, 8), 13 * 8);
    }
}
