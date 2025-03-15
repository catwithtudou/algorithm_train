pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|w| (w[1] as i32 - w[0] as i32).abs())
            .sum()
    }
}
