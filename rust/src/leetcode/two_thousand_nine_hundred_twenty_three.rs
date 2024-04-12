pub struct Solution;

impl Solution {
	pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
		let n = grid.len() as i32;
		for (i,row) in grid.iter().enumerate() {
			if row.iter().sum::<i32>() == n-1 {
				return i as _;
			}
		}
		unreachable!()
	}
}