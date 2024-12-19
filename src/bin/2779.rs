struct Solution;

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut ans = 0;
        let mut l = 0;

        for r in 0..nums.len() {
            while nums[r] - nums[l] > 2 * k {
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}

fn main() {
    let nums = vec![1, 5, 9, 7, 3];
    let k = 2;
    let result = Solution::maximum_beauty(nums, k);
    println!("{}", result); // Output should be 4
}
