pub struct Solution;


impl Solution {
	pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
		let (mut row_max, mut col_max) = (vec![0; grid.len()], vec![0; grid.len()]);
		for (i, row) in grid.iter().enumerate() {
			for (j, &x) in row.iter().enumerate() {
				row_max[i] = row_max[i].max(x);
				col_max[j] = col_max[j].max(x);
			}
		}

		let mut ans = 0;
		for (i, row) in grid.iter().enumerate() {
			for (j, &x) in row.iter().enumerate() {
				ans += row_max[i].min(col_max[j]) - x;
			}
		}
		ans
	}
}