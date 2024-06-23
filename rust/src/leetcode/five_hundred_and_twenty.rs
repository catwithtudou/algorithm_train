pub struct Solution;

impl Solution {
	pub fn detect_capital_use(word: String) -> bool {
		let up_cnt = word.chars().filter(|&c| c.is_uppercase()).count();
		up_cnt == 0 || up_cnt == word.len() || (up_cnt == 1 && word.chars().next().unwrap().is_uppercase())
	}
}