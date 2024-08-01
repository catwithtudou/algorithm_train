pub struct Solution;

impl Solution {
	pub fn maxmium_score(mut cards: Vec<i32>, cnt: i32) -> i32 {
		cards.sort_unstable_by(|a, b| b.cmp(a));
		let cnt = cnt as usize;
		let s = cards[..cnt].iter().sum::<i32>();
		if s % 2 == 0 {
			return s;
		}

		let replaces_num_func = |x: i32| -> i32{
			for &v in cards[cnt..].iter() {
				if v % 2 != x % 2 {
					return s - x + v;
				}
			}
			0
		};

		let x = cards[cnt - 1];
		let mut ans = replaces_num_func(x);
		for &v in cards[..cnt - 1].iter().rev() {
			if v % 2 != x % 2 {
				ans = ans.max(replaces_num_func(v));
				break;
			}
		}

		ans
	}
}