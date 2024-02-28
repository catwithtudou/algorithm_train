pub struct Solution;

impl Solution {
	pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
		let mut cost = cost;
		let mut ans = 0;
		for i in (1..=n as usize / 2).rev() {
			ans += (cost[i * 2 - 1] - cost[i * 2]).abs();
			cost[i - 1] += cost[i * 2 - 1].max(cost[i * 2]);
		}
		ans
	}
}