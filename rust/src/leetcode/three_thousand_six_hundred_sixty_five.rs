pub struct Solution;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let n = grid[0].len();
        let mut dp = vec![[0; 2]; n + 1];
        dp[1] = [1, 1];

        for row in &grid {
            for (j, &x) in row.iter().enumerate() {
                let left = dp[j][0];
                let top = dp[j + 1][1];

                dp[j + 1] = if x == 0 {
                    let ways = (left + top) % MOD;
                    [ways, ways]
                } else {
                    [top, left]
                };
            }
        }

        dp[n][0]
    }
}