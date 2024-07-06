pub struct Solution;

impl Solution {
	pub fn distribute_candies(n: i32, limit: i32) -> i32 {
		let c = |n| if n > 1 { n * (n - 1) / 2 } else { 0 };
		c(n + 2) - 3 * c(n - limit + 1) + 3 * c(n - 2 * limit) - c(n - 3 * limit - 1)
	}
}