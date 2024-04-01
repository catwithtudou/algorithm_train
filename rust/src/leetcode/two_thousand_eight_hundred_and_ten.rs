use std::collections::VecDeque;

pub struct Solution;

impl Solution {
	pub fn final_string(s: String) -> String {
		let mut q = VecDeque::new();
		let mut tail = true;
		for c in s.chars() {
			if c == 'i' {
				tail = !tail;
			} else if tail {
				q.push_back(c);
			} else {
				q.push_front(c);
			}
		}
		if tail {
			q.iter().collect()
		} else {
			q.iter().rev().collect()
		}
	}
}