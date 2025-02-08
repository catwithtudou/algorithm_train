pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut f = vec![vec![0; n + 1]; m + 1];
        f[0][1] = 1;
        for (i, row) in obstacle_grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x == 0 {
                    f[i + 1][j + 1] = f[i + 1][j] + f[i][j + 1];
                }
            }
        }
        f[m][n]
    }
}
