﻿pub struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let c2 = |n| {
            if n > 1 {
                n as i64 * (n as i64 - 1) / 2
            } else {
                0
            }
        };
        c2(n + 2) - 3 * c2(n - limit + 1) + 3 * c2(n - 2 * limit) - c2(n - 3 * limit - 1)
    }
}
