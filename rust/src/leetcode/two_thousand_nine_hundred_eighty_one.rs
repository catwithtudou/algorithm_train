pub struct Solution;

impl Solution {
	pub fn maximum_length(s: String) -> i32 {
		let mut groups = vec![vec![]; 26];
		let s = s.as_bytes();
		let mut cnt = 0;
		for (i, &c) in s.iter().enumerate() {
			cnt += 1;
			if i + 1 == s.len() || c != s[i + 1] {
				groups[(c - b'a') as usize].push(cnt);
				cnt = 0;
			}
		}

		let mut ans = 0;
		for a in groups.iter_mut() {
			if a.is_empty() {
				continue;
			}
			a.sort_unstable_by(|x, y| y.cmp(x));
			a.push(0);
			a.push(0);
			ans = ans.max(a[0] - 2).max(a[1].min(a[0] - 1)).max(a[2]);
		}

		if ans > 0 { ans } else { -1 }
	}
}