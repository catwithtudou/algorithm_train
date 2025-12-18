pub struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let k_usize = k as usize;

        // 使用 i64::MIN / 2 避免在状态转移计算时发生下溢 (underflow)
        let min_val = i64::MIN / 2;

        // 初始化 DP 表: dimensions [n+1][k+2][3]
        // f[i][j][state]
        let mut f = vec![vec![[min_val; 3]; k_usize + 2]; n + 1];

        // 初始化基础状态
        for j in 1..=k_usize + 1 {
            f[0][j][0] = 0;
        }

        for (i, &price) in prices.iter().enumerate() {
            let p = price as i64; // 将价格转为 i64 进行计算
            for j in 1..=k_usize + 1 {
                // 状态转移逻辑
                // f[i+1][j][0] = max(f[i][j][0], f[i][j][1]+p, f[i][j][2]-p)
                f[i + 1][j][0] = f[i][j][0].max(f[i][j][1] + p).max(f[i][j][2] - p);

                // f[i+1][j][1] = max(f[i][j][1], f[i][j-1][0]-p)
                f[i + 1][j][1] = f[i][j][1].max(f[i][j - 1][0] - p);

                // f[i+1][j][2] = max(f[i][j][2], f[i][j-1][0]+p)
                f[i + 1][j][2] = f[i][j][2].max(f[i][j - 1][0] + p);
            }
        }

        f[n][k_usize + 1][0]
    }
}
