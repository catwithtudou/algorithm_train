pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let length = strs.len();
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![vec![0; n + 1]; m + 1]; length + 1];

        for i in 1..=length {
            let (zeros, ones) = Self::get_zeros_ones(&strs[i - 1]);
            for j in 0..=m {
                for k in 0..=n {
                    dp[i][j][k] = dp[i - 1][j][k];
                    if j >= zeros as usize && k >= ones as usize {
                        dp[i][j][k] =
                            dp[i][j][k].max(dp[i - 1][j - zeros as usize][k - ones as usize] + 1);
                    }
                }
            }
        }
        dp[length][m][n]
    }

    fn get_zeros_ones(s: &str) -> (i32, i32) {
        let mut zeros = 0;
        let mut ones = 0;
        for c in s.chars() {
            if c == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        (zeros, ones)
    }
}
