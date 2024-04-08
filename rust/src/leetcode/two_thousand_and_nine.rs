pub struct Solution;

impl Solution {
	pub fn min_operations(mut nums: Vec<i32>) -> i32 {
		let n = nums.len() as i32;
		nums.sort_unstable();
		nums.dedup();
		let mut ans = 0;
		let mut left = 0;
		for (i, &x) in nums.iter().enumerate() {
			while nums[left] < x - n + 1 {
				left += 1;
			}
			ans = ans.max(i - left + 1);
		}

		n - ans as i32
	}
}