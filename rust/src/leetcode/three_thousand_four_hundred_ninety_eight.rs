pub struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.bytes()
            .enumerate()
            .map(|(i, c)| (b'{' - c) as i32 * (i as i32 + 1))
            .sum()
    }
}