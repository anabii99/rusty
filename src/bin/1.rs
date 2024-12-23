struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!(
        "Indices of the two numbers that add up to {}: {:?}",
        target, result
    );
}

// #[test]
// fn test() {
//     assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
//     assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
//     assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
// }
