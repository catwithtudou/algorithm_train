pub struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len();
        let mut ans = vec![0; n];
        for j in 0..n {
            let mut cur_col = j as i32;
            for row in &grid {
                let d = row[cur_col as usize];
                cur_col += d;
                if cur_col < 0 || cur_col as usize == n || row[cur_col as usize] != d {
                    cur_col = -1;
                    break;
                }
            }
            ans[j] = cur_col;
        }
        ans
    }
}
