use std::cmp::min;

struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut abs_sum: i64 = 0;
        let mut min_abs = i32::MAX;
        let mut odd_neg = 0;

        for row in matrix {
            for &num in &row {
                abs_sum += num.abs() as i64;
                min_abs = min(min_abs, num.abs());
                if num < 0 {
                    odd_neg ^= 1;
                }
            }
        }

        abs_sum - (odd_neg * min_abs as i64 * 2)
    }
}

fn main() {
    let matrix = vec![
        vec![1, -2, 3],
        vec![-4, 5, -6],
        vec![7, -8, 9]
    ];

    let result = Solution::max_matrix_sum(matrix);
    println!("{}", result);
}
