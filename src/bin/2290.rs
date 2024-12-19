use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let m = grid.len();
        let n = grid[0].len();

        let mut min_heap = BinaryHeap::new();
        min_heap.push(State {
            cost: grid[0][0],
            position: (0, 0),
        });

        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = grid[0][0];

        while let Some(State { cost, position }) = min_heap.pop() {
            let (i, j) = position;
            if i == m - 1 && j == n - 1 {
                return cost;
            }

            for &(di, dj) in &dirs {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                let new_dist = cost + grid[i][j];
                if new_dist < dist[x][y] {
                    dist[x][y] = new_dist;
                    min_heap.push(State {
                        cost: new_dist,
                        position: (x, y),
                    });
                }
            }
        }

        dist[m - 1][n - 1]
    }
}

struct Solution;
fn main() {
    let grid = vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 0, 0]];
    let solution = Solution::minimum_obstacles(grid);
    println!("Minimum obstacles: {}", solution);
}
