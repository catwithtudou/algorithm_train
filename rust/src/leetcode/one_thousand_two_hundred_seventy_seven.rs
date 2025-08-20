pub struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    dp[i + 1][j + 1] = dp[i][j + 1].min(dp[i + 1][j]).min(dp[i][j]) + 1;
                    ans += dp[i + 1][j + 1];
                }
            }
        }

        ans
    }
}
