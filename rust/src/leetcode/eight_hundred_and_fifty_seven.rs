use std::collections::BinaryHeap;

pub struct Solution;


impl Solution {
	pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
		let n = quality.len();
		let k = k as usize;
		let mut id = (0..n).collect::<Vec<_>>();
		id.sort_unstable_by(|&i, &j| (wage[i] * quality[j]).cmp(&(wage[j] * quality[i])));


		let mut h = BinaryHeap::new();
		let mut sum_q = 0;
		for i in 0..k {
			h.push(quality[id[i]]);
			sum_q += quality[id[i]];
		}

		let mut ans = sum_q as f64 * wage[id[k - 1]] as f64 / quality[id[k - 1]] as f64;

		for i in k..n {
			let q = quality[id[i]];
			if q < *h.peek().unwrap() {
				sum_q -= h.pop().unwrap() - q;
				h.push(q);
				ans = ans.min(sum_q as f64 * wage[id[i]] as f64 / q as f64);
			}
		}

		ans
	}
}