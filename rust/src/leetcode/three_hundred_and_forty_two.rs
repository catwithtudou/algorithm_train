pub struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0 && n % 3 == 1
    }
}
