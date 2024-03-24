pub struct Solution;


impl Solution {
	pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
		let mut dp = vec![amount + 1; amount as usize + 1];
		dp[0] = 0;
		for c in coins {
			(c as usize..=amount as usize).for_each(|i| {
				dp[i] = dp[i].min(dp[i - c as usize] + 1);
			});
		}

		if dp[amount as usize] == amount + 1 {
			-1
		} else {
			dp[amount as usize]
		}
	}
}