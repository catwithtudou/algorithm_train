pub struct Solution;

impl Solution {
	pub fn is_valid_serialization(preorder: String) -> bool {
		let mut stk: Vec<&str> = Vec::new();
		for s in preorder.split(',') {
			stk.push(s);
			while stk.len() >= 3 && stk[stk.len() - 1] == "#" && stk[stk.len() - 2] == "#" && stk[stk.len() - 3] != "#" {
				stk.pop();
				stk.pop();
				stk.pop();
				stk.push("#");
			}
		}

		stk.len() == 1 && stk.last().unwrap() == &"#"
	}
}