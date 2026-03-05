pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut diff = 0;
        for (i, ch) in s.chars().enumerate() {
            if (ch as usize - '0' as usize) != i % 2 {
                diff += 1;
            }
        }
        diff.min(s.len() as i32 - diff)
    }
}