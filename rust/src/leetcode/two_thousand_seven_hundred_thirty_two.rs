use std::collections::HashMap;

pub struct Solution;

impl Solution {
	pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
		let mut num_to_m: HashMap<i32, i32> = HashMap::new();
		for (i, row) in grid.iter().enumerate() {
			let mut mask = 0;
			for (j, &x) in row.iter().enumerate() {
				mask |= x << j;
			}
			if mask == 0 {
				return vec![i as i32];
			}
			num_to_m.insert(mask, i as i32);
		}

		for (&x, &i) in num_to_m.iter() {
			for (&y, &j) in num_to_m.iter() {
				if (x & y) == 0 {
					return vec![i.min(j), i.max(j)];
				}
			}
		}

		vec![]
	}
}