use std::collections::HashMap;
impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Helper function to find the complement of a number relative to the target.
        fn find_complement(x: i32, target: i32) -> i32 {
            target - x
        }

        // Use fold to iterate over numbers, but here we'll build a map and check for complements.
        nums.iter()
            .enumerate()
            .fold((HashMap::new(), Vec::new()), |(mut map, mut acc), (i, &num)| {
                let complement = find_complement(num, target);
                if let Some(&j) = map.get(&complement) {
                    acc = vec![j as i32, i as i32];
                } else {
                    map.insert(num, i);
                }
                (map, acc)
            })
            .1
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Indices of the two numbers that add up to {}: {:?}", target, result);
}
