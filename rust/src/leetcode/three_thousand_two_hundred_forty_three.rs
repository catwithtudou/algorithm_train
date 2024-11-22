pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut from: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut f: Vec<i32> = (0..n as i32).collect();
        let mut ans = vec![0; queries.len()];
        for (qi, q) in queries.iter().enumerate() {
            let (l, r) = (q[0] as usize, q[1] as usize);
            from[r].push(l);
            if (f[l] + 1) < f[r] {
                f[r] = f[l] + 1;
                for i in (r + 1)..n {
                    f[i] = min(f[i], f[i - 1] + 1);
                    for &j in &from[i] {
                        f[i] = min(f[i], f[j] + 1);
                    }
                }
            }
            ans[qi] = f[n - 1];
        }

        ans
    }
}
