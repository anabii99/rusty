struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut ans = Vec::new();
        let mut id = 0;
        let mut parity_ids = vec![id];

        for i in 1..nums.len() {
            if nums[i] % 2 == nums[i - 1] % 2 {
                id += 1;
            }
            parity_ids.push(id);
        }

        for query in queries {
            let from = query[0] as usize;
            let to = query[1] as usize;
            ans.push(parity_ids[from] == parity_ids[to]);
        }

        ans
    }
}

fn main() {
    let nums = vec![2, 4, 6, 3, 5, 7];
    let queries = vec![vec![0, 2], vec![1, 3], vec![2, 5]];

    let result = Solution::is_array_special(nums, queries);

    for res in result {
        println!("{}", res);
    }
}
