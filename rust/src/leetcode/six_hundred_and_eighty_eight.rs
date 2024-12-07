pub struct Solution;

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let dirs = [
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
        ];
        let mut memo = vec![vec![vec![0.0 as f64; n as usize]; n as usize]; k as usize + 1];

        fn dfs(
            i: i32,
            j: i32,
            k: i32,
            n: i32,
            dirs: &[(i32, i32)],
            memo: &mut Vec<Vec<Vec<f64>>>,
        ) -> f64 {
            if i < 0 || j < 0 || i >= n || j >= n {
                return 0.0;
            }
            if k == 0 {
                return 1.0;
            }

            if memo[k as usize][i as usize][j as usize] > 0.0 {
                return memo[k as usize][i as usize][j as usize];
            }

            let mut res = 0f64;
            for &(di, dj) in dirs {
                res += dfs(i + di, j + dj, k - 1, n, dirs, memo);
            }
            res /= 8.0;
            memo[k as usize][i as usize][j as usize] = res;
            res
        }

        dfs(row, column, k, n, &dirs, &mut memo)
    }
}
