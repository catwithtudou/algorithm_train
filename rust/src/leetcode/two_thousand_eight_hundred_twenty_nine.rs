pub struct Solution;

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let m = n.min(k / 2);
        (m * (m + 1) + (k * 2 + n - m - 1) * (n - m)) / 2
    }
}
