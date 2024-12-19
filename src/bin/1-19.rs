use std::collections::HashMap;

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        nums.iter().enumerate().try_fold(&mut map, |map, (second, num)| {
            let complement = target - num;
            match map.get(&complement) {
                Some(first) => Err(vec![*first as i32, second as i32]),
                None => {map.insert(num, second); Ok(map)}
            }
        }).err().unwrap()
    }
}


struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Indices of the two numbers that add up to {}: {:?}", target, result);
}