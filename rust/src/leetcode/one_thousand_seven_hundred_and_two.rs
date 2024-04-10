pub struct Solution;

impl Solution {
	pub fn maximum_binary_string(binary: String) -> String {
		if let Some(i) = binary.find('0') {
			let cnt_one = binary[i..].bytes().filter(|&c| c == b'1').count();
			return "1".repeat(binary.len() - 1 - cnt_one) + "0" + &"1".repeat(cnt_one);
		}
		binary
	}
}