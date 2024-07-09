use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
	pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
		let mut sx: BTreeMap<i32, i32> = BTreeMap::new();
		let mut sy: BTreeMap<i32, i32> = BTreeMap::new();
		for p in &points {
			let (sx_key, sy_key) = (p[0] - p[1], p[0] + p[1]);
			*sx.entry(sx_key).or_insert(0) += 1;
			*sy.entry(sy_key).or_insert(0) += 1;
		}

		let mut res = std::i32::MAX;
		for p in &points {
			let (sx_key, sy_key) = (p[0] - p[1], p[0] + p[1]);
			let count_sx = sx.entry(sx_key).or_insert(0);
			*count_sx -= 1;
			if *count_sx == 0 {
				sx.remove(&sx_key);
			}
			let count_sy = sy.entry(sy_key).or_insert(0);
			*count_sy -= 1;
			if *count_sy == 0 {
				sy.remove(&sy_key);
			}
			if !sx.is_empty() && !sy.is_empty() {
				let max_sx = *sx.iter().rev().next().unwrap().0 - *sx.iter().next().unwrap().0;
				let max_sy = *sy.iter().rev().next().unwrap().0 - *sy.iter().next().unwrap().0;
				res = res.min(max_sx.max(max_sy));
			}
			*sx.entry(sx_key).or_insert(0) += 1;
			*sy.entry(sy_key).or_insert(0) += 1;
		}
		res
	}
}