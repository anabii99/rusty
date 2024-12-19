use std::collections::HashMap;
struct Solution;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let i = i as i32; // Predefine the index as i32
            let complement = target - num;
            if let Some(&index) = num_map.get(&complement) {
                return vec![index, i];
            }
            num_map.insert(num, i);
        }

        vec![] // Return an empty vector if no solution is found
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result); // Output: [0, 1]
}
