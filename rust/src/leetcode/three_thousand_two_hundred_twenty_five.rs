pub struct Solution;

impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }

        let mut col_sum = vec![vec![0_i64; n + 1]; n];

        for j in 0..n {
            for i in 0..n {
                col_sum[j][i + 1] = col_sum[j][i] + grid[i][j] as i64;
            }
        }

        let mut f = vec![vec![[0_i64; 2]; n + 1]; n];

        for j in 0..n - 1 {
            // 避免同时可变和不可变借用 f
            let (left, right) = f.split_at_mut(j + 1);
            let cur = &left[j];
            let next = &mut right[0];

            // 用前缀最大值优化
            let mut pre_max = cur[0][1] - col_sum[j][0];

            for pre in 1..=n {
                next[pre][0] = cur[pre][0].max(pre_max + col_sum[j][pre]);
                next[pre][1] = next[pre][0];
                pre_max = pre_max.max(cur[pre][1] - col_sum[j][pre]);
            }

            // 用后缀最大值优化
            let mut suf_max = cur[n][0] + col_sum[j + 1][n];

            for pre in (1..n).rev() {
                next[pre][0] = next[pre][0].max(suf_max - col_sum[j + 1][pre]);
                suf_max = suf_max.max(cur[pre][0] + col_sum[j + 1][pre]);
            }

            // 单独计算 pre = 0 的状态
            next[0][0] = suf_max;
            next[0][1] = cur[0][0].max(cur[n][0]);
        }

        let mut ans = 0_i64;
        for state in &f[n - 1] {
            ans = ans.max(state[0]);
        }

        ans
    }
}