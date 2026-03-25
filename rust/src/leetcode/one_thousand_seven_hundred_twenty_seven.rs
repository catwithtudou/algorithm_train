pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut matrix = matrix;
        let mut max_area = 0;

        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i - 1][j];
                }
            }
        }

        for i in 0..m {
            matrix[i].sort_by(|a, b| b.cmp(a));
            for j in 0..n {
                let area = (j as i32 + 1) * matrix[i][j];
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }
}