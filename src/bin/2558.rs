use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut k = k;
        let mut pq = BinaryHeap::from(gifts);

        while let Some(v) = pq.pop() {
            let x = (v as f64).sqrt() as i32;
            pq.push(x);
            k -= 1;
            if k == 0 {
                break;
            }
        }

        pq.into_iter().map(|a| a as i64).sum::<i64>()
    }
}

fn main() {
    let gifts = vec![25, 64, 9, 4, 100];
    let k = 4;
    let result = Solution::pick_gifts(gifts, k);
    println!("The total sum of gifts is: {}", result);
}
