use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();

        for &a in arr.iter() {
            if seen.contains(&(a * 2)) || (a % 2 == 0 && seen.contains(&(a / 2))) {
                return true;
            }
            seen.insert(a);
        }

        false
    }
}

fn main() {
    let arr = vec![10, 2, 5, 3];
    let result = Solution::check_if_exist(arr);
    println!("{}", result); // Output: true
}
