pub struct Solution;

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        if grid.is_empty() || grid[0].is_empty() || k < 0 {
            return -1;
        }

        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        const NEG_INF: i32 = i32::MIN;

        let mut dp = vec![vec![vec![NEG_INF; k + 2]; n + 1]; m + 1];

        for used in 1..(k + 2) {
            dp[0][1][used] = 0;
        }

        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                for used in 0..=k {
                    let prev_used = if x > 0 {
                        used as isize - 1
                    } else {
                        used as isize
                    };

                    let prev_idx = (prev_used + 1) as usize;

                    let best_prev = dp[i][j + 1][prev_idx].max(dp[i + 1][j][prev_idx]);

                    dp[i + 1][j + 1][used + 1] = if best_prev == NEG_INF {
                        NEG_INF
                    } else {
                        best_prev.saturating_add(x)
                    };
                }
            }
        }

        let ans = dp[m][n][k + 1];
        if ans < 0 {
            -1
        } else {
            ans
        }
    }
}
