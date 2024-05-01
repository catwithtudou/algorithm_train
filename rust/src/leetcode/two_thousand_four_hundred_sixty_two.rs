use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
	pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
		let n = costs.len();
		let k = k as usize;
		let c = candidates as usize;
		if c * 2 + k > n {
			costs.sort_unstable();
			return costs[..k].iter().map(|&x| x as i64).sum();
		}

		let mut left = BinaryHeap::new();
		let mut right = BinaryHeap::new();
		for i in 0..c {
			left.push(-costs[i]);
			right.push(-costs[n - i - 1]);
		}

		let mut ans = 0;
		let mut i = c;
		let mut j = n - c - 1;
		for _ in 0..k {
			if left.peek().unwrap() >= right.peek().unwrap() {
				ans -= left.pop().unwrap() as i64;
				left.push(-costs[i]);
				i += 1;
			} else {
				ans -= right.pop().unwrap() as i64;
				right.push(-costs[j]);
				j -= 1;
			}
		}


		return ans;
	}
}