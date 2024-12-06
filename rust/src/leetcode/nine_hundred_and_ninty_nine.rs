pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (mut x0, mut y0) = (0, 0);
        for (i, row) in board.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 'R' {
                    x0 = i;
                    y0 = j;
                    break;
                }
            }
        }

        let mut ans = 0;
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let size = board.len() as i32;
        for dir in dirs {
            let mut x = x0 as i32 + dir.0;
            let mut y = y0 as i32 + dir.1;
            while x >= 0 && x < size && y >= 0 && y < size && board[x as usize][y as usize] == '.' {
                x += dir.0;
                y += dir.1;
            }
            if x >= 0 && x < size && y >= 0 && y < size && board[x as usize][y as usize] == 'p' {
                ans += 1;
            }
        }
        ans
    }
}
