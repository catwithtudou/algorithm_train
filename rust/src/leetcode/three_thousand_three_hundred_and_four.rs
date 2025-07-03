pub struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let ones_count = (k - 1).count_ones() as u8;
        (b'a' + ones_count) as char
    }
}
