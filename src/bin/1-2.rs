use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate().map(|(i, num)| (i as i32, num))
        {
          if let Some(&j) = map.get(&(target - num)) {
                return vec![j, i];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15, 100];
    let target = 115;
    let result = Solution::two_sum(nums, target);
    println!(
        "Indices of the two numbers that add up to {}: {:?}",
        target, result
    );
}
