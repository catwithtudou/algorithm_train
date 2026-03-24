pub struct Solution;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MOD: i64 = 12345;
        let m = grid.len();
        let n = grid[0].len();
        let mut p = vec![vec![0; n]; m];
        let mut suf = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                p[i][j] = suf as i32;
                suf = suf * grid[i][j] as i64 % MOD;
            }
        }

        let mut pre = 1;
        for i in 0..m {
            for j in 0..n {
                p[i][j] = (p[i][j] as i64 * pre % MOD) as i32;
                pre = pre * grid[i][j] as i64 % MOD;
            }
        }

        p
    }
}
