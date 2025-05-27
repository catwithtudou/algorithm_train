pub struct Solution;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        n * (n + 1) / 2 - n / m * (n / m + 1) * m
    }
}