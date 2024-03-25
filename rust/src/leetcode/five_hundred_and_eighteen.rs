pub struct Solution;

impl Solution {
	pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
		let mut dp = vec![0; amount as usize + 1];
		dp[0] = 1;
		for coin in coins {
			for i in 1..=amount {
				if coin > i {
					continue;
				}
				dp[i as usize] = dp[i as usize] + dp[(i - coin) as usize];
			}
		}


		dp[amount as usize]
	}
}