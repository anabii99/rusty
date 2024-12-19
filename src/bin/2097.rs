use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut in_out_degree: HashMap<i32, i32> = HashMap::new();

        // Build graph and count in/out degrees
        for pair in &pairs {
            let u = pair[0];
            let v = pair[1];
            adjacency_list.entry(u).or_insert_with(Vec::new).push(v);
            *in_out_degree.entry(u).or_insert(0) += 1; // out-degree
            *in_out_degree.entry(v).or_insert(0) -= 1; // in-degree
        }

        // Find starting node (head)
        let mut start_node = pairs[0][0];
        for (&node, &degree) in &in_out_degree {
            if degree == 1 {
                start_node = node;
                break;
            }
        }

        let mut path = Vec::new();
        let mut node_stack = Vec::new();
        node_stack.push(start_node);

        while let Some(current_node) = node_stack.last().copied() {
            if let Some(neighbors) = adjacency_list.get_mut(&current_node) {
                if neighbors.is_empty() {
                    path.push(node_stack.pop().unwrap());
                } else {
                    let next_node = neighbors.pop().unwrap();
                    node_stack.push(next_node);
                }
            } else {
                path.push(node_stack.pop().unwrap());
            }
        }

        let mut arrangement = Vec::with_capacity(path.len() - 1);

        for i in (1..path.len()).rev() {
            arrangement.push(vec![path[i], path[i - 1]]);
        }

        arrangement
    }
}

fn main() {
    let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
    let result = Solution::valid_arrangement(pairs);
    for pair in result {
        println!("{:?}", pair);
    }
}
