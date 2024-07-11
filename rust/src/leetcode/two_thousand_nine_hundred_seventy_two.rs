pub struct Solution;

impl Solution {
	pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
		let n = nums.len();
		let mut i = 0;
		while i < n - 1 && nums[i] < nums[i + 1] {
			i += 1;
		}
		if i == n - 1 {
			return n as i64 * (n + 1) as i64 / 2;
		}

		let mut i = i as i64;
		let mut ans = i + 2;
		let mut j = n - 1;
		while j == n - 1 || nums[j] < nums[j + 1] {
			while i >= 0 && nums[i as usize] >= nums[j] {
				i -= 1;
			}
			ans += i + 2;
			j -= 1;
		}
		ans
	}
}