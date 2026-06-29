pub struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut ans = 0;
        for pattern in patterns {
            if word.contains(&pattern) {
                ans += 1;
            }
        }
        ans as i32
    }
}