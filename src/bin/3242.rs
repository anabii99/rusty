impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph: Vec<_> = (0..n)
            .map(|idx| if idx == n - 1 { vec![] } else { vec![idx + 1] })
            .collect();
        let mut dist: Vec<_> = (0..n).map(|num| num as i32).collect();
        let mut tofix = Vec::new();
        queries
            .into_iter()
            .map(|query| {
                let (u, v) = (query[0] as usize, query[1] as usize);
                graph[u].push(v);
                tofix.push((u, v));
                while let Some((u, v)) = tofix.pop() {
                    if dist[v] > dist[u] + 1 {
                        dist[v] = dist[u] + 1;
                        tofix.extend(graph[v].iter().map(|&w| (v, w)))
                    }
                }
                dist[n - 1]
            })
            .collect()
    }
}

struct Solution;
fn main() {
    let n = 5;
    let queries = vec![
        vec![0, 2],
        vec![1, 3],
        vec![2, 4],
    ];

    let result = Solution::shortest_distance_after_queries(n, queries);
    println!("{:?}", result); // Output: [4, 4, 2]
}
