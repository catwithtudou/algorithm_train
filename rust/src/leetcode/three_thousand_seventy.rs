pub struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;
        let (m, n) = (grid.len(), grid[0].len());
        let mut sum = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + grid[i][j];
                if sum[i + 1][j + 1] <= k {
                    ans += 1;
                }
            }
        }

        ans
    }
} // TODO: add implementation for problem 3070.
