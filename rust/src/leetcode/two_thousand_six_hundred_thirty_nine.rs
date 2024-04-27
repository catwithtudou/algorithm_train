pub struct Solution;


impl Solution {
	pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
		let n = grid[0].len();
		let mut ans = vec![0; n];
		for j in 0..n {
			let (mut ma, mut mi) = (0, 0);
			for row in &grid {
				ma = ma.max(row[j]);
				mi = mi.min(row[j]);
			}
			let mut x_len = 1;
			let mut x = (ma / 10).max(-mi);
			while x > 0 {
				x_len += 1;
				x /= 10;
			}
			ans[j] = x_len;
		}
		ans
	}
}