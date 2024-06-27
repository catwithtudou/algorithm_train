pub struct Solution;


impl Solution {
	pub fn smallest_string(S: String) -> String {
		let mut s = S.into_bytes();
		let n = s.len();
		for i in 0..n {
			if s[i] <= b'a' {
				continue;
			}
			for j in i..n {
				if s[j] == b'a' {
					break;
				}
				s[j] -= 1;
			}
			return unsafe { String::from_utf8_unchecked(s) };
		}
		s[n - 1] = b'z';
		unsafe { String::from_utf8_unchecked(s) }
	}
}