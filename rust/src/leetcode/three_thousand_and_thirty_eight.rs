pub struct Solution;

impl Solution {
	pub fn max_operations(nums: Vec<i32>) -> i32 {
		let cnt = nums[0] + nums[1];
		for i in (3..nums.len()).step_by(2) {
			if nums[i - 1] + nums[i] != cnt {
				return (i / 2) as _;
			}
		}
		return (nums.len() / 2) as _;
	}
}