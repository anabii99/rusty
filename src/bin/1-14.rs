use std::collections::HashMap;

// Define a struct to hold the number and its index
#[derive(Debug)]
struct NumEntry {
    value: i32,
    index: usize,
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a HashMap to store each number and its index.
        let mut num_map: HashMap<i32, usize> = HashMap::new();
        
        // Create a vector of NumEntry structs
        let entries: Vec<NumEntry> = nums
            .iter()
            .enumerate()
            .map(|(i, &num)| NumEntry { value: num, index: i })
            .collect();

        // Iterate through the entries.
        for entry in entries.iter() {
            let complement = target - entry.value;

            // Check if the complement exists in the map.
            if let Some(&complement_index) = num_map.get(&complement) {
                // If found, return the indices of the two numbers.
                return vec![complement_index as i32, entry.index as i32];
            }

            // Store the current number and its index in the map.
            num_map.insert(entry.value, entry.index);
        }

        // Return an empty vector if no solution is found (should not happen according to problem statement).
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    let result = Solution::two_sum(nums, target);
    
    println!("{:?}", result); // Output: [0, 1]
}
