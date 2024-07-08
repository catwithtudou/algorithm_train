pub struct Solution;

impl Solution {
	pub fn pivot_index(nums: Vec<i32>) -> i32 {
		let sum = nums.iter().sum::<i32>();
		let mut left_s = 0;
		for (i, &x) in nums.iter().enumerate() {
			if left_s * 2 == sum - x {
				return i as _;
			}
			left_s += x;
		}
		-1
	}
}