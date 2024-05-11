pub struct Solution;

impl Solution {
	pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
		let mut ans = garbage[0].len() as i32;
		let mut sum_t = 0;
		let mut t_map = [0; 4];
		for i in 1..garbage.len() {
			let s = &garbage[i];
			ans += s.len() as i32;
			sum_t += travel[i - 1];
			for c in s.chars() {
				ans += sum_t - t_map[c as usize & 3];
				t_map[c as usize & 3] = sum_t;
			}
		}


		ans
	}
}