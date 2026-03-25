pub struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();

        // diag_sum[i+1][j+1] = diag_sum[i][j] + grid[i][j]  （主对角线方向）
        // anti_sum[i+1][j]   = anti_sum[i][j+1] + grid[i][j] （反对角线方向）
        let mut diag_sum = vec![vec![0i32; n + 1]; m + 1];
        let mut anti_sum = vec![vec![0i32; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                let v = grid[i][j];
                diag_sum[i + 1][j + 1] = diag_sum[i][j] + v;
                anti_sum[i + 1][j] = anti_sum[i][j + 1] + v;
            }
        }

        // queryDiagonal(x, y, k)     = diag_sum[x+k][y+k] - diag_sum[x][y]
        // queryAntiDiagonal(x, y, k) = anti_sum[x+k][y+1-k] - anti_sum[x][y+1]

        let mut x = 0i32;
        let mut y = 0i32;
        let mut z = 0i32;

        for i in 0..m {
            for j in 0..n {
                // 菱形大小为 0（单格）时直接更新
                Self::update(&mut x, &mut y, &mut z, grid[i][j]);

                // mx 为能向各方向延伸的最大步数
                let mx = i.min(m - 1 - i).min(j.min(n - 1 - j));

                for k in 1..=mx {
                    // 右上边: queryDiagonal(i-k, j, k)
                    //   = diag_sum[i][j+k] - diag_sum[i-k][j]
                    let a = diag_sum[i][j + k] - diag_sum[i - k][j];

                    // 左下边: queryDiagonal(i, j-k, k)
                    //   = diag_sum[i+k][j] - diag_sum[i][j-k]
                    let b = diag_sum[i + k][j] - diag_sum[i][j - k];

                    // 左上边: queryAntiDiagonal(i-k+1, j-1, k-1)
                    //   = anti_sum[i][j-k+1] - anti_sum[i-k+1][j]
                    let c = anti_sum[i][j - k + 1] - anti_sum[i - k + 1][j];

                    // 右下边: queryAntiDiagonal(i, j+k, k+1)
                    //   = anti_sum[i+k+1][j] - anti_sum[i][j+k+1]
                    let d = anti_sum[i + k + 1][j] - anti_sum[i][j + k + 1];

                    Self::update(&mut x, &mut y, &mut z, a + b + c + d);
                }
            }
        }

        // 去掉末尾多余的 0（不足三个不同值时）
        let mut ans = vec![x, y, z];
        while ans.last() == Some(&0) {
            ans.pop();
        }
        ans
    }

    /// 维护前三大的不同值（x >= y >= z）
    fn update(x: &mut i32, y: &mut i32, z: &mut i32, v: i32) {
        if v > *x {
            *z = *y;
            *y = *x;
            *x = v;
        } else if v < *x && v > *y {
            *z = *y;
            *y = v;
        } else if v < *y && v > *z {
            *z = v;
        }
    }
}