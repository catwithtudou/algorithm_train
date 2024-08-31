pub struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let check = |i: usize, j: usize| -> bool {
            let mut cnt = [0, 0];
            cnt[(grid[i][j] as u8 & 1) as usize] += 1;
            cnt[(grid[i][j + 1] as u8 & 1) as usize] += 1;
            cnt[(grid[i + 1][j] as u8 & 1) as usize] += 1;
            cnt[(grid[i + 1][j + 1] as u8 & 1) as usize] += 1;
            cnt[0]!= 2
        };

        check(0, 0) || check(0, 1) || check(1, 0) || check(1, 1)
    }


}
