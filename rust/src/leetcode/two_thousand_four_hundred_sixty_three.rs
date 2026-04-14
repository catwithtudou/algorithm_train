pub struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        factory.sort_unstable_by_key(|f| f[0]);
        robot.sort_unstable();

        let n = factory.len();
        let m = robot.len();
        let inf = i64::MAX / 4;

        // dp[i][j] 表示前 i 个工厂修理前 j 个机器人 的最小总代价
        let mut dp = vec![vec![0_i64; m + 1]; n + 1];

        // 0 个工厂无法修理任何正数个机器人
        for j in 1..=m {
            dp[0][j] = inf;
        }

        for i in 0..n {
            let position = factory[i][0] as i64;
            let limit = factory[i][1] as usize;

            for j in 0..m {
                // 当前工厂不修机器人
                let mut res = dp[i][j + 1];

                // 当前工厂修 k 个机器人
                let mut dist_sum = 0_i64;
                let upper = (j + 1).min(limit);
                for k in 1..=upper {
                    dist_sum += (robot[j - k + 1] as i64 - position).abs();
                    res = res.min(dp[i][j - k + 1] + dist_sum);
                }

                dp[i + 1][j + 1] = res;
            }
        }

        dp[n][m]
    }
}