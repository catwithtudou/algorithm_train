pub struct Solution;


impl Solution {
	pub fn cal_points(operations: Vec<String>) -> i32 {
		let mut st = vec![];
		for op in operations {
			match op.as_bytes()[0] {
				b'+' => st.push(st[st.len() - 2] + st[st.len() - 1]),
				b'D' => st.push(st[st.len() - 1] * 2),
				b'C' => { st.pop(); }
				_ => st.push(op.parse::<i32>().unwrap())
			}
		}
		st.iter().sum()
	}
}