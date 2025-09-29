use std::i32;

pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut f = vec![vec![0; n]; n];
        for i in (0..n - 2).rev() {
            for j in i + 2..n {
                f[i][j] = i32::MAX;
                for k in i + 1..j {
                    f[i][j] = f[i][j].min(f[i][k] + f[k][j] + values[i] * values[k] * values[j]);
                }
            }
        }
        f[0][n - 1]
    }
}
