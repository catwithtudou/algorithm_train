pub struct Solution;

impl Solution {
	pub fn get_hint(secret: String, guess: String) -> String {
		let (mut a, mut b) = (0, 0);
		let (mut cnt_g, mut cnt_s) = (vec![0; 10], vec![0; 10]);
		for (s, g) in secret.bytes().zip(guess.bytes()) {
			if s == g {
				a += 1;
			} else {
				cnt_g[(g - b'0') as usize] += 1;
				cnt_s[(s - b'0') as usize] += 1;
			}
		}
		let b = cnt_s.into_iter().zip(cnt_g).map(|(s, g)| s.min(g)).sum::<i32>();
		format!("{}A{}B", a, b)
	}
}