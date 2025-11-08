pub struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let k = 32 - n.leading_zeros();
        (1 << k) - 1 - Self::minimum_one_bit_operations(n - (1 << (k - 1)))
    }
}
