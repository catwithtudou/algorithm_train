pub struct Solution;

impl Solution {
	pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
		let n = graph.len();
		let mut vis = vec![false; n];
		let mut initial_bool = vec![false; n];
		for &x in &initial {
			initial_bool[x as usize] = true
		}

		let mut ans = -1;
		let mut max_size = 0;
		for &x in &initial {
			if vis[x as usize] {
				continue;
			}
			let mut node_id = -1;
			let mut size = 0;
			Self::dfs(x as usize, &graph, &mut vis, &initial_bool, &mut node_id, &mut size);
			if node_id >= 0 && (size > max_size || size == max_size && node_id < ans) {
				ans = node_id;
				max_size = size;
			}
		}


		if ans < 0 { *initial.iter().min().unwrap() } else { ans }
	}

	fn dfs(x: usize, graph: &Vec<Vec<i32>>, vis: &mut Vec<bool>, initial_bool: &Vec<bool>, node_id: &mut i32, size: &mut i32) {
		vis[x] = true;
		*size += 1;
		if *node_id != -2 && initial_bool[x] {
			*node_id = if *node_id < 0 { x as i32 } else { -2 };
		}
		for (i, &v) in graph[x].iter().enumerate() {
			if v == 1 && !vis[i] {
				Self::dfs(i, graph, vis, initial_bool, node_id, size);
			}
		}
	}
}