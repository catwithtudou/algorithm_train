pub struct Solution;


impl Solution {
	pub fn minimum_levels(possible: Vec<i32>) -> i32 {
		let s = possible.iter().sum::<i32>() * 2 - possible.len() as i32;
		let mut pre = 0;
		for i in 0..possible.len() - 1 {
			pre += if possible[i] == 1 { 2 } else { -2 };
			if pre > s {
				return (i + 1) as _;
			}
		}

		-1
	}
}