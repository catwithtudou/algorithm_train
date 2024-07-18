use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
	pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
		let n = n as usize;
		let mut g = vec![vec![]; n];
		for e in edges {
			let x = e[0] as usize;
			let y = e[1] as usize;
			let wt = e[2];
			g[x].push((y, wt));
			g[y].push((x, wt));
		}
		let mut dis = vec![-1; n];
		dis[0] = 0;
		let mut h = BinaryHeap::new();
		h.push((0, 0));
		while let Some((dx, x)) = h.pop() {
			if -dx > dis[x] {
				continue;
			}
			for &(y, d) in &g[x] {
				let new_dis = -dx + d;
				if new_dis < disappear[y] && (dis[y] < 0 || new_dis < dis[y]) {
					dis[y] = new_dis;
					h.push((-new_dis, y));
				}
			}
		}


		dis
	}
}