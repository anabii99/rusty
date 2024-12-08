use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        const M: usize = 2;
        const N: usize = 3;
        const GOAL: &str = "123450";

        let mut start = String::new();

        // Hash the 2D vector into a string.
        for i in 0..M {
            for j in 0..N {
                start.push(char::from_digit(board[i][j] as u32, 10).unwrap());
            }
        }

        if start == GOAL {
            return 0;
        }

        let mut q = VecDeque::new();
        q.push_back(start.clone());
        let mut seen = HashSet::new();
        seen.insert(start);

        let mut step = 1;
        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let s = q.pop_front().unwrap();
                let zero_index = s.find('0').unwrap();
                let i = zero_index / N;
                let j = zero_index % N;
                for &(dx, dy) in &DIRS {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if x < 0 || x == M as i32 || y < 0 || y == N as i32 {
                        continue;
                    }
                    let swapped_index = (x * N as i32 + y) as usize;
                    let mut new_s = s.clone();
                    new_s.replace_range(zero_index..zero_index + 1, &s[swapped_index..swapped_index + 1]);
                    new_s.replace_range(swapped_index..swapped_index + 1, &s[zero_index..zero_index + 1]);
                    if new_s == GOAL {
                        return step;
                    }
                    if !seen.contains(&new_s) {
                        q.push_back(new_s.clone());
                        seen.insert(new_s);
                    }
                }
            }
            step += 1;
        }

        -1
    }
}

fn main() {
    let board = vec![vec![1, 2, 3], vec![4, 0, 5]];
    let solution = Solution::sliding_puzzle(board);
    println!("{}", solution);
}
