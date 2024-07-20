pub struct Solution;


impl Solution {
	pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
		let mut from: Vec<(usize, usize)> = Vec::new();
		let mut to: Vec<(usize, usize)> = Vec::new();
		for (i, row) in grid.iter().enumerate() {
			for (j, &col) in row.iter().enumerate() {
				if col > 1 {
					for _ in 1..col {
						from.push((i, j));
					}
				} else if col == 0 {
					to.push((i, j));
				}
			}
		}

		let mut ans = i32::MAX;
		let mut from = from.clone();
		Self::permute(&mut from, 0, &to, &mut ans);
		ans
	}

	fn permute(a: &mut Vec<(usize, usize)>, i: usize, to: &Vec<(usize, usize)>, ans: &mut i32) {
		if i == a.len() {
			let mut total = 0;
			for (i, p) in a.iter().enumerate() {
				total += (p.0 as i32 - to[i].0 as i32).abs() + (p.1 as i32 - to[i].1 as i32).abs();
			}
			*ans = (*ans).min(total);
			return;
		}

		Self::permute(a, i + 1, to, ans);
		for j in i + 1..a.len() {
			a.swap(i, j);
			Self::permute(a, i + 1, to, ans);
			a.swap(i, j);
		}
	}
}