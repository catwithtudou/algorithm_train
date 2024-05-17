pub struct Solution;


impl Solution {
	pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
		let mut jobs = difficulty.into_iter().zip(profit.into_iter()).collect::<Vec<_>>();
		jobs.sort_unstable_by(|a, b| a.0.cmp(&b.0));
		worker.sort_unstable();
		let mut ans = 0;
		let (mut j, mut max_profit) = (0, 0);
		for w in worker {
			while j < jobs.len() && jobs[j].0 <= w {
				max_profit = max_profit.max(jobs[j].1);
				j += 1;
			}
			ans += max_profit;
		}

		ans
	}
}