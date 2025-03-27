pub struct Solution;

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let s  = s.as_bytes();
        let n = s.len();
        let mut ans = 0;

        for i in 1..n {
            if s[i-1] != s[i] {
                ans += i.min(n-i) as i64;
            }
        }

        ans
    }
}