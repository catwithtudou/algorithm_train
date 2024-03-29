pub struct Solution;

impl Solution {
	pub fn minimum_sum(nums: Vec<i32>) -> i32 {
		let n = nums.len();
		let mut suf = vec![0; n];
		suf[n - 1] = nums[n - 1];
		for i in (2..n - 1).rev() {
			suf[i] = suf[i + 1].min(nums[i]);
		}

		let mut ans = i32::MAX;
		let mut pre = nums[0];
		for i in 1..n - 1 {
			if pre < nums[i] && nums[i] > suf[i + 1] {
				ans = ans.min(pre + nums[i] + suf[i + 1]);
			}
			pre = pre.min(nums[i]);
		}


		if ans == i32::MAX {
			-1
		} else {
			ans
		}
	}
}