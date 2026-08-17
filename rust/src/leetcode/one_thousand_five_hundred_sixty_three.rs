pub struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();

        // 前缀和
        let mut prefix = vec![0_i32; n + 1];
        for (i, &v) in stone_value.iter().enumerate() {
            prefix[i + 1] = prefix[i] + v;
        }

        // dp[i][j] 表示区间 [i, j) 上 Alice 能获得的最大分数
        let mut dp = vec![vec![0_i32; n + 1]; n];

        for i in (0..n - 1).rev() {
            for j in i + 2..=n {
                for k in i + 1..j {
                    let left_sum = prefix[k] - prefix[i];
                    let right_sum = prefix[j] - prefix[k];

                    let score = if left_sum < right_sum {
                        dp[i][k] + left_sum
                    } else if left_sum > right_sum {
                        dp[k][j] + right_sum
                    } else {
                        dp[i][k].max(dp[k][j]) + left_sum
                    };

                    dp[i][j] = dp[i][j].max(score);
                }
            }
        }

        dp[0][n]
    }
}