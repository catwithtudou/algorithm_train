pub struct Solution;


impl Solution {
	pub fn capitalize_title(title: String) -> String {
		title.split(" ")
			.map(|s| if s.len() > 2 {
				s[..1].to_uppercase() + &s[1..].to_lowercase()
			} else {
				s.to_lowercase()
			})
			.collect::<Vec<_>>()
			.join(" ")
	}
}