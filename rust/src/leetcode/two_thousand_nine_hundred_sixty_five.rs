pub struct Solution;

impl Solution {
	pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
		let n = grid.len();
		let mut cnt = vec![0; n * n + 1];

		for row in grid {
			for x in row {
				cnt[x as usize] += 1;
			}
		}

		let mut ans = vec![0; 2];

		for i in 1..=n * n {
			if cnt[i] == 2 {
				ans[0] = i as i32;
			} else if cnt[i] == 0 {
				ans[1] = i as i32;
			}
		}

		ans
	}
}