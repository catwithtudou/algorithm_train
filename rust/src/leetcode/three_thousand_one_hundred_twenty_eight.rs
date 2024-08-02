pub struct Solution;

impl Solution {
	pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
		let n = grid[0].len();
		let col_sum = (0..n).map(|j| grid.iter().map(|row| row[j]).sum::<i32>() - 1).collect::<Vec<_>>();
		grid.iter()
			.map(|row|
				(row.iter().sum::<i32>() - 1) as i64 *
					row.iter()
						.zip(col_sum.iter())
						.filter_map(|(&x, &c)| (x != 0).then_some(c))
						.sum::<i32>() as i64
			).sum()
	}
}