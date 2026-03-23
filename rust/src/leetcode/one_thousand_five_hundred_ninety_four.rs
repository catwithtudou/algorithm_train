pub struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        const MOD: i64 = 1_000_000_007;

        // f[i][j] = (到达(i,j)的最小乘积, 到达(i,j)的最大乘积)
        let mut f = vec![vec![(0_i64, 0_i64); n]; m];

        for i in 0..m {
            for j in 0..n {
                let x = grid[i][j] as i64;

                if i == 0 && j == 0 {
                    f[0][0] = (x, x);
                    continue;
                }

                let mut res_min = i64::MAX;
                let mut res_max = i64::MIN;

                if i > 0 {
                    let (mn, mx) = f[i - 1][j];
                    res_min = res_min.min(mn * x).min(mx * x);
                    res_max = res_max.max(mn * x).max(mx * x);
                }

                if j > 0 {
                    let (mn, mx) = f[i][j - 1];
                    res_min = res_min.min(mn * x).min(mx * x);
                    res_max = res_max.max(mn * x).max(mx * x);
                }

                f[i][j] = (res_min, res_max);
            }
        }

        let ans = f[m - 1][n - 1].1;
        if ans < 0 {
            -1
        } else {
            (ans % MOD) as i32
        }
    }
}