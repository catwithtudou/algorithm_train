pub struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // row_sum: m x (n+1)
        let mut row_sum = vec![vec![0i32; n + 1]; m];
        // col_sum: (m+1) x n
        let mut col_sum = vec![vec![0i32; n]; m + 1];
        // diag_sum: (m+1) x (n+1)   (top-left -> bottom-right)
        let mut diag_sum = vec![vec![0i32; n + 1]; m + 1];
        // anti_sum: (m+1) x (n+1)   (top-right -> bottom-left)
        let mut anti_sum = veca s da s da s da s d![vec![0i32; n + 1]; m + 1];

        // build prefix sums
        for i in 0..m {
            for j in 0..n {
                let x = grid[i][j];
                row_sum[i][j + 1] = row_sum[i][j] + x;
                col_sum[i + 1][j] = col_sum[i][j] + x;
                diag_sum[i + 1][j + 1] = diag_sum[i][j] + x;
                anti_sum[i + 1][j] = anti_sum[i][j + 1] + x;
            }
        }

        let mut k = m.min(n);
        while k >= 1 {
            for i in k..=m {
                'next: for j in k..=n {
                    // main diagonal sum
                    let sum = diag_sum[i][j] - diag_sum[i - k][j - k];

                    // anti diagonal sum must match
                    if anti_sum[i][j - k] - anti_sum[i - k][j] != sum {
                        continue;
                    }

                    // each row sum must match
                    for r in (i - k)..i {
                        if row_sum[r][j] - row_sum[r][j - k] != sum {
                            continue 'next;
                        }
                    }

                    // each col sum must match
                    for c in (j - k)..j {
                        if col_sum[i][c] - col_sum[i - k][c] != sum {
                            continue 'next;
                        }
                    }

                    return k as i32;
                }
            }

            if k == 1 {
                break;
            }
            k -= 1;
        }

        1
    }
}
