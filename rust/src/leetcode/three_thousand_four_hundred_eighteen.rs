pub struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();
        let neg_inf = i32::MIN / 2;


        let mut f = vec![vec![vec![neg_inf; 3]; n + 1]; m + 1];

        f[0][1][0] = 0;

        for (i, row) in coins.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                f[i + 1][j + 1][0] =
                    f[i + 1][j][0].max(f[i][j + 1][0]) + x;

                f[i + 1][j + 1][1] = (f[i + 1][j][1] + x)
                    .max(f[i][j + 1][1] + x)
                    .max(f[i + 1][j][0].max(f[i][j + 1][0]));

                f[i + 1][j + 1][2] = (f[i + 1][j][2] + x)
                    .max(f[i][j + 1][2] + x)
                    .max(f[i + 1][j][1].max(f[i][j + 1][1]));
            }
        }

        *f[m][n].iter().max().unwrap()
    }
}
