pub struct Solution;

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort_unstable_by_key(|a| a[1]);
        let n = events.len();
        let k = k as usize;
        let mut f = vec![vec![0; k + 1]; n + 1];

        for (i, event) in events.iter().enumerate() {
            let p = events[..i].partition_point(|a| a[1] < event[0]);
            for j in 1..=k {
                f[i + 1][j] = f[i][j].max(f[p][j - 1] + event[2]);
            }
        }

        f[n][k]
    }
}
