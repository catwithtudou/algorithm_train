pub struct Solution;

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        const MASK: i32 = 0x55555555;
        vec![(n & MASK).count_ones() as i32, (n & MASK << 1).count_ones() as i32]
    }
}
