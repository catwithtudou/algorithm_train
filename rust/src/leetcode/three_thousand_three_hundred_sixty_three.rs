use std::vec;

pub struct Solution;

impl Solution {
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();

        let dp = |grid: &[Vec<i32>]| -> i32 {
            let mut f = vec![vec![0; n + 1]; n - 1];

            f[0][n - 1] = grid[0][n - 1];

            for i in 1..n - 1 {
                let start_j = (n - 1 - i).max(i + 1);
                for j in start_j..n {
                    let mut max_val = f[i - 1][j];
                    if j > 0 {
                        max_val = max_val.max(f[i - 1][j - 1]);
                    }
                    if j + 1 <= n {
                        max_val = max_val.max(f[i - 1][j + 1]);
                    }
                    f[i][j] = max_val + grid[i][j];
                }
            }

            f[n - 2][n - 1]
        };

        let mut ans = 0;

        for (i, row) in fruits.iter().enumerate() {
            ans += row[i];
        }

        ans += dp(&fruits);

        for i in 0..n {
            for j in 0..i {
                fruits[j][i] = fruits[i][j];
            }
        }

        ans + dp(&fruits)
    }
}
