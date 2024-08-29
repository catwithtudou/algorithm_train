pub struct Solution;


impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        grid[0].windows(2).all(|col| col[0] != col[1])
            && grid.windows(2).all(|row| row[0] == row[1])
    }
}
