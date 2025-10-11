pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut cnt = HashMap::new();
        for &x in &power {
            *cnt.entry(x).or_insert(0) += 1;
        }

        let n = cnt.len();
        let mut a: Vec<i32> = cnt.keys().copied().collect();
        a.sort_unstable();

        let mut f = vec![0i64; n + 1];
        let mut j = 0;
        for i in 0..n {
            let x = a[i];
            while j < n && a[j] < x - 2 {
                j += 1;
            }
            f[i + 1] = f[i].max(f[j] + x as i64 * cnt[&x] as i64);
        }
        f[n]
    }
}
