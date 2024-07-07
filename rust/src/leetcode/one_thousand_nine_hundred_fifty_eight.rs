pub struct Solution;

impl Solution {
	pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
		let (m, n) = (board.len() as i32, board[0].len() as i32);
		for (dx, dy) in [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)] {
			let (mut x, mut y) = (r_move + dx, c_move + dy);
			if x < 0 || x >= m || y < 0 || y >= n || board[x as usize][y as usize] == '.' || board[x as usize][y as usize] == color {
				continue;
			}
			loop {
				x += dx;
				y += dy;
				if x < 0 || x >= m || y < 0 || y >= n || board[x as usize][y as usize] == '.' {
					break;
				}
				if board[x as usize][y as usize] == color {
					return true;
				}
			}
		}
		false
	}
}