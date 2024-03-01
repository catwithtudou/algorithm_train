pub struct Solution;


impl Solution {
	pub fn valid_partition(nums: Vec<i32>) -> bool {
		let n = nums.len();
		let mut f = vec![false; n + 1];
		f[0] = true;
		for i in 1..n {
			if f[i - 1] && (nums[i] == nums[i - 1]) {
				f[i + 1] = true;
				continue;
			}

			if i > 1 && f[i - 2] && ((nums[i] == nums[i - 1] && nums[i] == nums[i - 2]) || (nums[i] == nums[i - 1] + 1 && nums[i - 1] == nums[i - 2] + 1)) {
				f[i + 1] = true;
				continue;
			}
		}

		f[n]
	}
}