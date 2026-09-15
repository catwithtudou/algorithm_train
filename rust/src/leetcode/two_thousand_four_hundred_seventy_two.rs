pub struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        fn is_palindrome(s: &[u8]) -> bool {
            let n = s.len();

            for i in 0..n / 2 {
                if s[i] != s[n - 1 - i] {
                    return false;
                }
            }

            true
        }

        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;

        let mut dp = vec![0; n + 1];

        for i in k..=n {
            dp[i] = dp[i - 1];

            if is_palindrome(&s[i - k..i]) {
                dp[i] = dp[i].max(dp[i - k] + 1);
            }

            if i > k && is_palindrome(&s[i - k - 1..i]) {
                dp[i] = dp[i].max(dp[i - k - 1] + 1);
            }
        }

        dp[n]
    }
}