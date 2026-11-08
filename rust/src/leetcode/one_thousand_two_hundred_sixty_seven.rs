pub struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_cnt = vec![0; grid.len()];
        let mut col_cnt = vec![0; grid[0].len()];

        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x > 0 {
                    row_cnt[i] += 1;
                    col_cnt[j] += 1;
                }
            }
        }

        let mut ans = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x > 0 && (row_cnt[i] > 1 || col_cnt[j] > 1) {
                    ans += 1;
                }
            }
        }
        ans
    }
}