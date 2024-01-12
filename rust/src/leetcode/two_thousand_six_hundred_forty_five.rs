pub struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let s = word.as_bytes();
        let mut ans = s[0] as i32 - *s.last().unwrap() as i32 + 2;
        for i in 1..s.len() {
            ans += (s[i] as i32 - s[i - 1] as i32 + 2) % 3
        }
        ans
    }
}