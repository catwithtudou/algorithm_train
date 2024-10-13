pub struct Solution;

use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n + 1];

        for i in 1..=n {
            f[i] = i32::MAX;
            for j in 1..=i {
                f[i] = min(f[i], max(j as i32, f[i - j] + 1));
            }
        }

        return f[n];
    }
}
