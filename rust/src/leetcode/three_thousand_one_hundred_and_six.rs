pub struct Solution;

impl Solution {
	pub fn get_smallest_string(s: String, mut k: i32) -> String {
		let mut s = s.into_bytes();
		for c in s.iter_mut() {
			let dis = (*c - b'a').min(b'z' - *c + 1) as i32;
			if dis > k {
				*c -= k as u8;
				break;
			}
			*c = b'a';
			k -= dis;
		}
		unsafe { String::from_utf8_unchecked(s) }
	}
}