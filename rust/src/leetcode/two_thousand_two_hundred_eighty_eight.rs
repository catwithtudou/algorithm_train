pub struct Solution;

impl Solution {
	pub fn discount_prices(sentence: String, discount: i32) -> String {
		let d = 1.0 - discount as f64 / 100.0;
		sentence.split_whitespace()
			.map(|w| {
				if w.starts_with('$') {
					if let Ok(price) = w[1..].parse::<u64>() {
						return format!("${:.2}", price as f64 * d);
					}
				}
				w.to_string()
			})
			.collect::<Vec<_>>()
			.join(" ")
	}
}