pub struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut diff_row = 0;
        for row in &grid {
            for j in 0..n / 2 {
                diff_row += if row[j] != row[n - 1 - j] { 1 } else { 0 };
            }
        }

        let mut diff_col = 0;
        for j in 0..n {
            for i in 0..m / 2 {
                diff_col += if grid[i][j] != grid[m - 1 - i][j] {
                    1
                } else {
                    0
                };
            }
        }

        diff_row.min(diff_col)
    }
}
