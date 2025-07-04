pub struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut inc = 0;
        let mut k = k;

        for i in (0..64 - (k - 1).leading_zeros() as usize).rev() {
            if k > 1 << i {
                inc += operations[i];
                k -= 1 << i;
            }
        }

        (b'a' + (inc % 26) as u8) as char
    }
}
