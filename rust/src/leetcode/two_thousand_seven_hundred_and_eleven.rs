use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![vec![0; n]; m];

        for k in 1..(m + n) {
            let min_j = (n as i32 - k as i32).max(0) as usize;
            let max_j = ((m + n - 1) as i32 - k as i32).min(n as i32 - 1) as usize;

            let mut set = HashSet::new();
            for j in min_j..=max_j {
                let i = k + j - n;
                ans[i][j] = set.len() as i32;
                set.insert(grid[i][j]);
            }

            let mut set = HashSet::new();
            for j in (min_j..=max_j).rev() {
                let i = k + j - n;
                ans[i][j] = (ans[i][j] - set.len() as i32).abs();
                set.insert(grid[i][j]);
            }
        }

        ans
    }
}
