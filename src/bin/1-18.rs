use std::collections::HashMap;
impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if map.contains_key(&complement) {
                let index = map.get(&complement).unwrap();
                return vec![*index, i as i32];
            }

            map.insert(num, i as i32);
        }

        vec![]
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Indices of the two numbers that add up to {}: {:?}", target, result);
}