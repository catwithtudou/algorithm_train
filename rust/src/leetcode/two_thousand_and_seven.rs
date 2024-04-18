use std::collections::HashMap;

pub struct Solution;


impl Solution {
	pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
		changed.sort_unstable();
		let mut count = HashMap::new();
		for num in &changed {
			*count.entry(*num).or_insert(0) += 1;
		}

		let mut res = Vec::new();
		for a in changed {
			if *count.get(&a).unwrap_or(&0) == 0 {
				continue;
			}
			*count.get_mut(&a).unwrap() -= 1;

			if *count.get(&(a * 2)).unwrap_or(&0) == 0 {
				return vec![];
			}
			*count.get_mut(&(a * 2)).unwrap() -= 1;
			res.push(a);
		}

		res
	}
}