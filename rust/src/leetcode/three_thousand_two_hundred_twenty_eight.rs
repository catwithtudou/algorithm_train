pub struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut ans = 0;
        let mut cnt1 = 0;
        let s = s.as_bytes();

        for (i, &c) in s.iter().enumerate() {
            if c == b'1' {
                cnt1 += 1;
            } else if i > 0 && s[i - 1] == b'1' {
                ans += cnt1;
            }
        }

        ans
    }
}
