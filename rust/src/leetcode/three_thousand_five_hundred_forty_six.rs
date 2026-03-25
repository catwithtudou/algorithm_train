pub struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid.is_empty() || grid[0].is_empty() {
            return false;
        }

        let total: i64 = grid.iter().flatten().map(|&x| x as i64).sum();

        Self::check(&grid, total) || Self::check(&Self::rotate_grid(&grid), total)
    }

    fn check(grid: &[Vec<i32>], total: i64) -> bool {
        if grid.len() <= 1 {
            return false;
        }

        let mut prefix_sum = 0_i64;

        for row in &grid[..grid.len() - 1] {
            prefix_sum += row.iter().map(|&x| x as i64).sum::<i64>();
            if prefix_sum * 2 == total {
                return true;
            }
        }

        false
    }

    fn rotate_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut rotated = vec![vec![0; m]; n];

        for (i, row) in grid.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                rotated[j][m - 1 - i] = value;
            }
        }

        rotated
    }
}