pub struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let n = s.len();
        let m = (32 - k.leading_zeros()) as usize;
        if n < m {
            return n as _;
        }

        let suf_val = i32::from_str_radix(&s[n - m..], 2).unwrap();
        let ans = if suf_val <= k { m } else { m - 1 };
        (ans + s[..n - m].bytes().filter(|&c| c == b'0').count()) as _
    }
}
