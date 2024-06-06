pub struct Solution;

impl Solution {
	pub fn minimum_steps(s: String) -> i64 {
		let mut ans = 0i64;
		let mut cnt = 0;
		for c in s.bytes() {
			if c == b'1' {
				cnt += 1;
			} else {
				ans += cnt;
			}
		}
		ans
	}
}