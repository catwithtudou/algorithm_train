pub struct Solution;

impl Solution {
	pub fn maximum_odd_binary_number(s: String) -> String {
		let mut cnt_one = s.chars().filter(|&c| c == '1').count();
		"1".repeat(cnt_one - 1) + &"0".repeat(s.len() - cnt_one) + "1"
	}
}