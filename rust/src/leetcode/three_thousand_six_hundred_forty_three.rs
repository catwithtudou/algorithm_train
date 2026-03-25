pub struct Solution;

impl Solution {
    pub fn reverse_submatrix(grid:  Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
                let mut grid = grid;


        let mut l = x;
        let mut r = x + k - 1;

        while l < r {
            for j in y..y + k {
                let temp = grid[l][j];
                grid[l][j] = grid[r][j];
                grid[r][j] = temp;
            }
            l += 1;
            r -= 1;
        }

        grid
    }
}
