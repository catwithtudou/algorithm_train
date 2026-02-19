pub struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut pre = 0;
        let mut cur = 0;
        for i in 0..s.len() {
            cur += 1;
            if i == s.len() - 1 || s[i] != s[i + 1] {
                ans += pre.min(cur);
                pre = cur;
                cur = 0;
            }
        }
        ans
    }
}
