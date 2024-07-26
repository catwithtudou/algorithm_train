use std::collections::HashSet;

pub struct Solution;

impl Solution {
	pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
		let mut set = nums.into_iter().collect::<HashSet<_>>();
		for (f, t) in move_from.into_iter().zip(move_to) {
			set.remove(&f);
			set.insert(t);
		}
		let mut ans = set.into_iter().collect::<Vec<_>>();
		ans.sort_unstable();
		ans
	}
}