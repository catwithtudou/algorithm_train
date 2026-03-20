pub struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0i32; n - k + 1]; m - k + 1];

        for i in 0..ans.len() {
            for j in 0..ans[0].len() {
                let mut a: Vec<i32> = grid[i..i + k]
                    .iter()
                    .flat_map(|row| &row[j..j + k])
                    .copied()
                    .collect();
                a.sort_unstable();

                let mut res = i32::MAX;
                for p in 1..k * k {
                    if a[p] > a[p - 1] {
                        res = res.min(a[p] - a[p - 1]);
                    }
                }

                if res < i32::MAX {
                    ans[i][j] = res;
                }
            }
        }

        ans
    }
}