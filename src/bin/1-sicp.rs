use std::collections::HashMap;

// Helper function to find the complement in a HashMap
fn find_complement(complement: i32, map: &HashMap<i32, usize>) -> Option<&usize> {
    map.get(&complement)
}

// Struct to hold the result of the two sum search
struct TwoSumResult {
    found: bool,
    indices: Vec<usize>,
    map: HashMap<i32, usize>,
}

// Function to find two numbers that sum to the target
fn find_two_sum(nums: &[i32], target: i32) -> TwoSumResult {
    nums.iter()
        .enumerate()
        .fold(
            TwoSumResult { found: false, indices: Vec::new(), map: HashMap::new() },
            |mut acc, (index, &num)| {
                let complement = target - num;
                if let Some(&complement_index) = find_complement(complement, &acc.map) {
                    if complement_index != index {
                        acc.found = true;
                        acc.indices = vec![complement_index, index];
                        return acc;
                    }
                }
                acc.map.insert(num, index);
                acc
            }
        )
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let result = find_two_sum(&nums, target);
        if result.found {
            result.indices.iter().map(|&x| x as i32).collect()
        } else {
            panic!("No two sum solution");
        }
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Indices of the two numbers that add up to {}: {:?}", target, result);
}