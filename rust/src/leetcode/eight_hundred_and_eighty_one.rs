pub struct Solution;

impl Solution {
	pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
		people.sort_unstable();
		let mut ans = 0;
		let (mut l, mut r) = (0, people.len() as i32 - 1);
		while l <= r {
			if people[l as usize] + people[r as usize] <= limit {
				l += 1;
			}
			r -= 1;
			ans += 1;
		}
		ans
	}
}