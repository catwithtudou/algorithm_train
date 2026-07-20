pub struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let k = k as usize % (m * n);
        let mut ans = vec![vec![0; n]; m];
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                let p = (i * n + j + k) % (m * n);
                ans[p / n][p % n] = x;
            }
        }
        ans
    }
}
