use std::collections::HashSet;

pub struct Solution;


impl Solution {
	pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
		let set = candy_type.iter().collect::<HashSet<_>>();
		set.len().min(candy_type.len() / 2) as _
	}
}