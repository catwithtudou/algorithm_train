pub struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;
        for i in 0..m / 2 {
            for j in 0..n / 2 {
                let cnt1 = grid[i][j]
                    + grid[i][n - 1 - j]
                    + grid[m - 1 - i][j]
                    + grid[m - 1 - i][n - 1 - j];
                ans += cnt1.min(4 - cnt1);
            }
        }

        if m % 2 == 1 && n % 2 == 1 {
            ans += grid[m / 2][n / 2];
        }

        let (mut diff, mut cnt1) = (0, 0);

        if m % 2 == 1 {
            for j in 0..n / 2 {
                if grid[m / 2][j] != grid[m / 2][n - 1 - j] {
                    diff += 1;
                } else {
                    cnt1 += grid[m / 2][j] * 2;
                }
            }
        }

        if n % 2 == 1 {
            for i in 0..m / 2 {
                if grid[i][n / 2] != grid[m - 1 - i][n / 2] {
                    diff += 1;
                } else {
                    cnt1 += grid[i][n / 2] * 2;
                }
            }
        }

        ans + if diff != 0 { diff } else { cnt1 % 4 }
    }
}
