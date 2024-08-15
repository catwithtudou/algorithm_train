use std::i32;

pub struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![vec![i32::MAX; n + 1]; m + 1];
        let mut ans = i32::MIN;

        for i in 0..m {
            for j in 0..n {
                let x = grid[i][j];
                let mn = f[i + 1][j].min(f[i][j + 1]);
                ans = ans.max(x - mn);
                f[i + 1][j + 1] = x.min(mn);
            }
        }

        ans
    }
}
