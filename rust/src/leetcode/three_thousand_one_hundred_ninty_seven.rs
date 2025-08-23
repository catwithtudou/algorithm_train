pub struct Solution;

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = i32::MAX;

        let solve = |grid: &[Vec<i32>], ans: &mut i32| {
            let m = grid.len();
            let n = grid[0].len();

            // 垂直切割：分成3个水平条带
            if m >= 3 {
                for i in 1..m {
                    for j in (i + 1)..m {
                        let area1 = Self::minimum_area_ii(&grid[..i], 0, n);
                        let area2 = Self::minimum_area_ii(&grid[i..j], 0, n);
                        let area3 = Self::minimum_area_ii(&grid[j..], 0, n);
                        *ans = (*ans).min(area1 + area2 + area3);
                    }
                }
            }

            // 混合切割
            if m >= 2 && n >= 2 {
                for i in 1..m {
                    for j in 1..n {
                        // 上面一块，下面分成左右两块
                        let area1 = Self::minimum_area_ii(&grid[..i], 0, n);
                        let area2 = Self::minimum_area_ii(&grid[i..], 0, j);
                        let area3 = Self::minimum_area_ii(&grid[i..], j, n);
                        *ans = (*ans).min(area1 + area2 + area3);

                        // 左边分成上下两块，右边一块
                        let area1 = Self::minimum_area_ii(&grid[..i], 0, j);
                        let area2 = Self::minimum_area_ii(&grid[..i], j, n);
                        let area3 = Self::minimum_area_ii(&grid[i..], 0, n);
                        *ans = (*ans).min(area1 + area2 + area3);
                    }
                }
            }
        };

        // 处理原始grid
        solve(&grid, &mut ans);

        // 处理旋转90度的grid
        let rotated = Self::rotate(&grid);
        solve(&rotated, &mut ans);

        ans
    }

    fn minimum_area_ii(grid: &[Vec<i32>], left_col: usize, right_col: usize) -> i32 {
        let mut left = right_col;
        let mut right = 0;
        let mut top = grid.len();
        let mut bottom = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row[left_col..right_col].iter().enumerate() {
                if x == 1 {
                    left = left.min(j);
                    right = right.max(j);
                    top = top.min(i);
                    bottom = i;
                }
            }
        }

        if left > right || top > bottom {
            return 0;
        }

        ((right - left + 1) * (bottom - top + 1)) as i32
    }

    fn rotate(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut rotated = vec![vec![0; m]; n];

        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                rotated[j][m - 1 - i] = x;
            }
        }

        rotated
    }
}
