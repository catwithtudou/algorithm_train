pub struct Solution;

impl Solution {
	pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
		let mut ans = vec![0; word.len()];
		let mut cur = 0;
		for (i, &w) in word.as_bytes().iter().enumerate() {
			cur = (cur * 10 + (w - b'0') as i64) % m as i64;
			if cur == 0 {
				ans[i] = 1;
			}
		}

		ans
	}
}