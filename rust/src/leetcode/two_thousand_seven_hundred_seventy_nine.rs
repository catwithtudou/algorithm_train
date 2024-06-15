pub struct Solution;

impl Solution {
	pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
		let mut ans = 0;
		let mut l = 0;
		nums.sort_unstable();
		for r in 0..nums.len() {
			if nums[r] - nums[l] > 2 * k {
				l += 1;
			}
			ans = ans.max((r - l) as i32 + 1);
		}
		ans
	}
}