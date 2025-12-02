use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut cnt: HashMap<i32, i64> = HashMap::with_capacity(points.len());

        for p in points {
            *cnt.entry(p[1]).or_insert(0) += 1;
        }

        let mut ans: i64 = 0;
        let mut s: i64 = 0;

        for &c in cnt.values() {
            let k = c * (c - 1) / 2;
            ans = (ans + (s % MOD) * (k % MOD)) % MOD;
            s = (s + k) % MOD;
        }

        ans as i32
    }
}
