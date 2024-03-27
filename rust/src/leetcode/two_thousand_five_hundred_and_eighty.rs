pub struct Solution;

impl Solution {
	pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
		ranges.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
		let mut ans = 1;
		let mut max_r = -1;
		for r in &ranges {
			if r[0] > max_r {
				ans = ans * 2 % 1_000_000_007;
			}
			max_r = max_r.max(r[1]);
		}

		ans
	}
}