pub struct Solution;

impl Solution {
	pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
		let mut res = nums[0] as i64;
		let mut dp: Vec<i64> = vec![i32::MIN as i64, i32::MIN as i64];
		dp[(nums[0] % 2) as usize] = nums[0] as i64;
		for i in 1..nums.len() {
			let j = (nums[i] % 2) as usize;
			let cur = (dp[j] + nums[i] as i64).max(dp[j ^ 1] - x as i64 + nums[i] as i64);
			res = res.max(cur);
			dp[j] = dp[j].max(cur);
		}
		res
	}
}