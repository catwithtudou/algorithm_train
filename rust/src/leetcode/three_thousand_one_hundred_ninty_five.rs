pub struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut left, mut right) = (i32::MAX, 0);
        let (mut top, mut bottom) = (i32::MAX, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    left = left.min(j as i32);
                    right = right.max(j as i32);
                    top = top.min(i as i32);
                    bottom = bottom.max(i as i32);
                }
            }
        }
        (right - left + 1) * (bottom - top + 1)
    }
}
