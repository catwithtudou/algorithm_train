pub struct Solution;

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.push(0);
        cuts.push(n);
        cuts.sort_unstable();

        let m = cuts.len();
        let mut f = vec![vec![0; m]; m];
        for i in (0..m - 2).rev() {
            for j in i + 2..m {
                let mut res = i32::MAX;
                for k in i + 1..j {
                    res = res.min(f[i][k] + f[k][j]);
                }
                f[i][j] = res + cuts[j] - cuts[i];
            }
        }
        f[0][m - 1]
    }
}
