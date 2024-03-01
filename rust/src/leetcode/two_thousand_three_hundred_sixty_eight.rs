pub struct Solution;


impl Solution {
	pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
		let mut g: Vec<Vec<i32>> = vec![vec![]; n as usize];
		for v in edges {
			let a = v[0] as usize;
			let b = v[1] as usize;
			g[a].push(b as i32);
			g[b].push(a as i32);
		}

		let mut is_restricted: Vec<i32> = vec![0; n as usize];
		for v in restricted {
			is_restricted[v as usize] = 1;
		}

		let mut seen: Vec<i32> = vec![0; n as usize];
		let mut count = 0;
		let mut s: Vec<i32> = vec![0];
		while !s.is_empty() {
			let cur = s.pop().unwrap() as usize;
			if seen[cur] == 1 {
				continue;
			}
			seen[cur] = 1;
			if is_restricted[cur] == 1 {
				continue;
			}
			count += 1;
			for &v in &g[cur] {
				s.push(v);
			}
		}

		count
	}
}