pub struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let (mut ans, mut cnt) = (1, 1);
        let s = s.as_bytes();
        for i in 1..s.len() {
            if s[i - 1] + 1 == s[i] {
                cnt += 1;
                ans = ans.max(cnt);
            } else {
                cnt = 1;
            }
        }
        ans
    }
}
