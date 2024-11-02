pub struct Solution;

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut a = Vec::with_capacity(m * n);
        let mut s = vec![vec![0; n + 1]; m + 1];
        for (i, row) in matrix.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                s[i + 1][j + 1] = s[i + 1][j] ^ s[i][j + 1] ^ s[i][j] ^ x;
            }
            a.extend_from_slice(&s[i + 1][1..n + 1]);
        }
        a.select_nth_unstable(m * n - k as usize);
        a[m * n - k as usize]
    }
}
