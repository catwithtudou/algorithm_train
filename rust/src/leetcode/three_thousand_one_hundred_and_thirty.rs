pub struct Solution;

const MOD: i32 = 1000000007;

impl Solution {
	pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
		let mut dp = vec![vec![vec![0; 2]; one as usize + 1]; zero as usize + 1];

		for i in 0..=zero as usize {
			for j in 0..=one as usize {
				for last_bit in 0..=1 {
					if i == 0 {
						if last_bit == 0 || j > limit as usize {
							dp[i][j][last_bit] = 0;
						} else {
							dp[i][j][last_bit] = 1;
						}
					} else if j == 0 {
						if last_bit == 1 || i > limit as usize {
							dp[i][j][last_bit] = 0;
						} else {
							dp[i][j][last_bit] = 1;
						}
					} else if last_bit == 0 {
						dp[i][j][last_bit] = dp[i - 1][j][0] + dp[i - 1][j][1];
						if i > limit as usize {
							dp[i][j][last_bit] -= dp[i - (limit as usize) - 1][j][1];
						}
					} else {
						dp[i][j][last_bit] = dp[i][j - 1][0] + dp[i][j - 1][1];
						if j > limit as usize {
							dp[i][j][last_bit] -= dp[i][j - (limit as usize) - 1][0];
						}
					}
					dp[i][j][last_bit] %= MOD;
					if dp[i][j][last_bit] < 0 {
						dp[i][j][last_bit] += MOD;
					}
				}
			}
		}

		return (dp[zero as usize][one as usize][0] + dp[zero as usize][one as usize][1]) % MOD;
	}
}
