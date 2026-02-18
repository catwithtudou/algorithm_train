pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let x = (n>>1) ^ n;
        (x+1) & x == 0
    }
}