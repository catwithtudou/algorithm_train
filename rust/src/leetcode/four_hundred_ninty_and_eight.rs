pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut res = Vec::with_capacity(m * n);
        for k in 0..m + n - 1 {
            let min_j = k.saturating_sub(m - 1);
            let max_j = k.min(n - 1);
            if k % 2 == 0 {
                for i in min_j..=max_j {
                    res.push(mat[k - i][i]);
                }
            } else {
                for i in (min_j..=max_j).rev() {
                    res.push(mat[k - i][i]);
                }
            }
        }
        res
    }
}
