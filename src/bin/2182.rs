struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut ans = String::new();
        let mut count = vec![0; 26];

        // Count the frequency of each character
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        loop {
            let add_one = !ans.is_empty() && Self::should_add_one(&ans, &count);
            let i = Self::get_largest_char(&ans, &count);
            if i == -1 {
                break;
            }
            // Convert repeat_limit to usize for comparison with count[i as usize]
            let repeats = if add_one { 1 } else { std::cmp::min(count[i as usize] as usize, repeat_limit as usize) };
            ans.push_str(&String::from_utf8(vec![b'a' + i as u8; repeats]).unwrap());
            count[i as usize] -= repeats as i32;
        }

        ans
    }

    fn should_add_one(ans: &str, count: &[i32]) -> bool {
        for i in (0..26).rev() {
            if count[i] > 0 {
                return ans.chars().last().unwrap() as u8 == (b'a' + i as u8);
            }
        }
        false
    }

    fn get_largest_char(ans: &str, count: &[i32]) -> i32 {
        for i in (0..26).rev() {
            if count[i] > 0 && (ans.is_empty() || ans.chars().last().unwrap() as u8 != (b'a' + i as u8)) {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    let s = String::from("aababab");
    let repeat_limit = 2;
    let result = Solution::repeat_limited_string(s, repeat_limit);
    println!("Result: {}", result); // Should print something like "ababaab"
}