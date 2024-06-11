pub struct Solution;


impl Solution {
	pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
		let mut ans = 0;
		for (i, row) in board.iter().enumerate() {
			for (j, &c) in row.iter().enumerate() {
				if c == 'X' && (j == 0 || row[j - 1] != 'X') && (i == 0 || board[i - 1][j] != 'X') {
					ans += 1;
				}
			}
		}
		ans
	}
}