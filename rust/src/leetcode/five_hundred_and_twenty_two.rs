pub struct Solution;

impl Solution {
	pub fn is_subseq(s: &[u8], t: &str) -> bool {
		let mut i = 0;
		for c in t.bytes() {
			if s[i] == c {
				i += 1;
				if i == s.len() {
					return true;
				}
			}
		}

		false
	}


	pub fn find_lu_slength(strs: Vec<String>) -> i32 {
		let mut ans = -1;
		'next:
		for (i, s) in strs.iter().enumerate() {
			if s.len() as i32 <= ans {
				continue;
			}

			let s = s.as_bytes();

			for (j, t) in strs.iter().enumerate() {
				if j != i && Self::is_subseq(s, t) {
					continue 'next;
				}
			}
			ans = s.len() as i32;
		}
		ans
	}
}