use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Wrapper to allow f64 to be used in a max-heap
#[derive(PartialEq)]
struct MaxNonNan(f64);
impl Eq for MaxNonNan {}
impl PartialOrd for MaxNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for MaxNonNan {
    fn cmp(&self, other: &MaxNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Solution;

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        // Closure to calculate the extra pass ratio
        let diff_ratio = |pass: i32, total: i32| -> MaxNonNan {
            let rate = pass as f64 / total as f64;
            let new_rate = (pass as f64 + 1.0) / (total as f64 + 1.0);
            MaxNonNan(new_rate - rate)
        };

        // Initialize the max-heap with the extra pass ratios
        let mut max_heap = classes
            .into_iter()
            .map(|i| {
                let pass = i[0];
                let total = i[1];
                (diff_ratio(pass, total), pass, total)
            })
            .collect::<BinaryHeap<_>>();

        // Calculate the initial sum of pass rates
        let n = max_heap.len() as f64;
        let sum: f64 = max_heap
            .iter()
            .map(|&(_, pass, total)| pass as f64 / total as f64)
            .sum();

        // Process each extra student
        let extra_sum: f64 = (0..extra_students)
            .map(|_| {
                let (MaxNonNan(gained), pass, total) = max_heap.pop().unwrap();
                max_heap.push((diff_ratio(pass + 1, total + 1), pass + 1, total + 1));
                gained
            })
            .sum();

        // Calculate the final average ratio
        (sum + extra_sum) / n
    }
}

fn main() {
    let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
    let extra_students = 2;
    let result = Solution::max_average_ratio(classes, extra_students);
    println!("Max average ratio: {}", result);
}