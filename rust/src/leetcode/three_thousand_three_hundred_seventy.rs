pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let len = 32 - n.leading_zeros();
        (1 << len) - 1
    }
}
