pub struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut ans = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 1..chars.len() {
            if chars[i].to_ascii_lowercase() != chars[i - 1].to_ascii_lowercase() {
                ans += 1;
            }
        }

        ans
    }
}
