pub struct Solution;

impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut graph: Vec<Vec<(i32, i64)>> = vec![vec![]; n + 1];
        for r in &rides {
            graph[r[1] as usize].push((r[0], (r[1] - r[0] + r[2]) as i64));
        }

        let mut f: Vec<i64> = vec![0; n + 1];
        for i in 2..=n {
            f[i] = f[i - 1];
            for r in &graph[i] {
                f[i] = f[i].max(f[r.0 as usize] + r.1);
            }
        }
        f[n]
    }
}