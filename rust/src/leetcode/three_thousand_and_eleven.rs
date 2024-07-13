pub struct Solution;

impl Solution {
	pub fn can_sort_array(nums: Vec<i32>) -> bool {
		let (mut i, n, mut pre_max) = (0, nums.len(), 0);
		while i < n {
			let mut mx = 0;
			let ones = nums[i].count_ones();
			while i < n && ones == nums[i].count_ones() {
				if nums[i] < pre_max {
					return false;
				}
				mx = mx.max(nums[i]);
				i += 1;
			}
			pre_max = mx;
		}
		true
	}
}