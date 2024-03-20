pub struct Solution;

impl Solution {
	pub fn min_non_zero_product(p: i32) -> i32 {
		const MOD: i64 = 1_000_000_007;

		fn pow(mut x: i64, p: i32) -> i64 {
			x %= MOD;
			let mut res = 1;
			for _ in 0..p {
				res = res * x % MOD;
				x = x * x % MOD;
			}

			res
		}

		let k = (1 << p) - 1;
		(k % MOD * pow(k - 1, p - 1) % MOD) as _
	}
}