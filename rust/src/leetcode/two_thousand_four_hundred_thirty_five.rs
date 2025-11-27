pub struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![vec![vec![0; k as usize]; n + 1]; m + 1];
        f[0][1][0] = 1;
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                for s in 0..k as usize {
                    let new_s = (s + x as usize) % k as usize;
                    f[i + 1][j + 1][s] = (f[i][j + 1][new_s] + f[i + 1][j][new_s]) % 1_000_000_007;
                }
            }
        }
        f[m][n][0]
    }
}
