pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let n = s.len();
        let m = t.len();

        if n < m {
            return 0;
        }

        // dp[i][j]:
        // s 的前 i 个字符中，有多少种方式组成 t 的前 j 个字符
        let mut dp = vec![vec![0_i64; m + 1]; n + 1];

        for i in 0..=n {
            dp[i][0] = 1;
        }

        for i in 0..n {
            let start = m.saturating_sub(n - i);
            let end = (i + 1).min(m);

            for j in start..end {
                // 不选择 s[i]
                dp[i + 1][j + 1] = dp[i][j + 1];

                // 选择 s[i] 与 t[j] 匹配
                if s[i] == t[j] {
                    dp[i + 1][j + 1] += dp[i][j];
                }
            }
        }

        dp[n][m] as i32
    }
}