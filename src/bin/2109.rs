impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::new();
        let mut j = 0; // spaces' index

        for (i, ch) in s.chars().enumerate() {
            if j < spaces.len() && i as i32 == spaces[j] {
                result.push(' ');
                j += 1;
            }
            result.push(ch);
        }

        result
    }
}

struct Solution;

fn main() {
    let s = String::from("LeetcodeHelpsMeLearn");
    let spaces = vec![8, 13, 15];
    let result = Solution::add_spaces(s, spaces);
    println!("{}", result); // Output: "Leetcode Helps Me Learn"
}
