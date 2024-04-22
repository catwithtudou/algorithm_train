pub struct Solution;

impl Solution {
	pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
		fn dfs(i: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
			if i == 0 {
				return 1;
			}

			if memo[i] != -1 {
				return memo[i];
			}


			let mut res = 0;
			for &x in nums {
				let x = x as usize;
				if x <= i {
					res += dfs(i - x, nums, memo);
				}
			}
			memo[i] = res;
			res
		}
		let t = target as usize;
		let mut memo = vec![-1; t + 1];
		dfs(t, &nums, &mut memo)
	}
}