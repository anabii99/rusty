use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn add_leaf(
            node: &Option<Rc<RefCell<TreeNode>>>,
            leaves: &mut Vec<Rc<RefCell<TreeNode>>>,
            roots: &mut HashMap<i32, Rc<RefCell<TreeNode>>>,
            unique_vals: &mut HashSet<i32>,
        ) {
            if let Some(ref n) = node {
                let val = n.borrow().val;
                unique_vals.insert(val);
                if roots.contains_key(&val) {
                    leaves.push(Rc::clone(n));
                }
            }
        }

        fn valid_bst(
            node: Option<Rc<RefCell<TreeNode>>>,
            min_val: Option<i32>,
            max_val: Option<i32>,
            visited: &mut HashSet<i32>,
        ) -> bool {
            if let Some(n) = node {
                let val = n.borrow().val;
                if !visited.insert(val)
                    || (min_val.is_some() && val <= min_val.unwrap())
                    || (max_val.is_some() && val >= max_val.unwrap())
                {
                    return false;
                }
                valid_bst(n.borrow().left.clone(), min_val, Some(val), visited)
                    && valid_bst(n.borrow().right.clone(), Some(val), max_val, visited)
            } else {
                true
            }
        }

        let mut unique_vals = HashSet::new();
        let mut roots = HashMap::new();
        let mut leaves = Vec::new();

        // Insert trees into roots map and add unique values
        for tree in &trees {
            if let Some(ref root) = tree {
                let val = root.borrow().val;
                roots.insert(val, Rc::clone(root));
                unique_vals.insert(val);
            }
        }

        // Collect leaves and add unique values
        for tree in &trees {
            if let Some(ref root) = tree {
                let val = root.borrow().val;
                unique_vals.insert(val);
                add_leaf(&root.borrow().left, &mut leaves, &mut roots, &mut unique_vals);
                add_leaf(&root.borrow().right, &mut leaves, &mut roots, &mut unique_vals);
            }
        }

        // Merge leaves into roots if valid BST candidates
        for leaf in leaves {
            let val = leaf.borrow().val;
            if let Some(root) = roots.get(&val) {
                let mut leaf_borrow = leaf.borrow_mut();
                leaf_borrow.left = root.borrow().left.clone();
                leaf_borrow.right = root.borrow().right.clone();
                roots.remove(&val);
            }
        }

        // Validate and check for single root in final tree
        let root_node = roots.values().next().cloned()?;
        let mut visited = HashSet::new();
        if roots.len() == 1
            && valid_bst(Some(root_node.clone()), None, None, &mut visited)
            && visited.len() == unique_vals.len()
        {
            Some(root_node)
        } else {
            None
        }
    }
}

// Helper function to build a tree from a vector of values
fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < values.len() {
        if let Some(node) = queue.pop_front() {
            if i < values.len() && values[i].is_some() {
                let left_child = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().left = Some(left_child.clone());
                queue.push_back(left_child);
            }
            i += 1;

            if i < values.len() && values[i].is_some() {
                let right_child = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().right = Some(right_child.clone());
                queue.push_back(right_child);
            }
            i += 1;
        }
    }

    Some(root)
}

// Helper function to convert vector of vectors into trees
fn build_trees(tree_vecs: Vec<Vec<i32>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    tree_vecs.into_iter()
        .map(|vec| build_tree(vec.into_iter().map(Some).collect()))
        .collect()
}

fn main() {
    // Example input
    let trees = vec![vec![2, 1], vec![3, 2, 5], vec![5, 4]];
    let tree_nodes = build_trees(trees);
    
    // Call can_merge and get the result
    let merged_tree = Solution::can_merge(tree_nodes);

    // Print output in level-order
    match merged_tree {
        Some(root) => println!("{:?}", level_order(root)),
        None => println!("Cannot merge the trees into a valid BST"),
    }
}

// Additional function to output the tree in level-order for verification
fn level_order(root: Rc<RefCell<TreeNode>>) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(Some(root));

    while let Some(Some(node)) = queue.pop_front() {
        result.push(Some(node.borrow().val));
        if node.borrow().left.is_some() || node.borrow().right.is_some() {
            queue.push_back(node.borrow().left.clone());
            queue.push_back(node.borrow().right.clone());
        }
    }
    result
}