pub struct Solution;

impl Solution {
	pub fn find_lu_slength(a: String, b: String) -> i32 {
		if a == b {
			-1
		} else {
			a.len().max(b.len()) as _
		}
	}
}