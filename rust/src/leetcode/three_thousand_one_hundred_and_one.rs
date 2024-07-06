pub struct Solution;

impl Solution {
	pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
		let (mut ans, mut cnt) = (0, 0);
		for i in 0..nums.len() {
			if i > 0 && nums[i] != nums[i - 1] {
				cnt += 1;
			} else {
				cnt = 1;
			}
			ans += cnt;
		}
		ans
	}
}