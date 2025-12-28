pub struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut i, mut j) = (0, n  as i32 - 1);
        let mut ans = 0;

        while i < m && j >= 0 {
            if grid[i][j as usize] < 0 {
                ans += (m - i) as i32;
                j -= 1;
            } else {
                i += 1;
            }
        }

        ans
    }
}
