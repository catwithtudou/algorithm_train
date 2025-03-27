pub struct Solution;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut mask:u32 = 0;

        for c in s.chars() {
            mask |= 1<<(c as u32 - 'a' as u32);
        }

        mask.count_ones() as i32
    }
}