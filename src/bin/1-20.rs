use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(&j) => return vec![j as i32, i as i32],
                None => {
                    map.insert(num, i);
                }
            }
        }

        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = Solution::two_sum(nums, target);
    match result.as_slice() {
        [i, j] => println!("Indices: {}, {}", i, j),
        _ => println!("No two sum solution"),
    }
}

