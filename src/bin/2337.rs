impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let start_chars: Vec<char> = start.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();

        let mut i = 0;
        let mut j = 0;
        let n = start_chars.len();
        let m = target_chars.len();

        // Count non-underscore characters in start and target
        let start_count = start_chars.iter().filter(|&&c| c != '_').count();
        let target_count = target_chars.iter().filter(|&&c| c != '_').count();

        // If the counts do not match, return false
        if start_count != target_count {
            return false;
        }

        while i < n && j < m {
            // Skip underscores in start
            while i < n && start_chars[i] == '_' {
                i += 1;
            }
            // Skip underscores in target
            while j < m && target_chars[j] == '_' {
                j += 1;
            }

            // Check if we have reached the end of both strings
            if i == n || j == m {
                return i == n && j == m;
            }

            // Check if the current characters match
            if start_chars[i] != target_chars[j] {
                return false;
            }

            // Check the conditions for 'R' and 'L'
            match start_chars[i] {
                'R' if i > j => return false,
                'L' if i < j => return false,
                _ => {}
            }

            i += 1;
            j += 1;
        }

        true
    }
}

struct Solution;
fn main() {
    let start = "_L".to_string();
    let target = "LL".to_string();
    println!("{}", Solution::can_change(start, target)); // Output: false
}
