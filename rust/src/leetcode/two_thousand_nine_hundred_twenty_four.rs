pub struct Solution;

impl Solution {
	pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
		let mut loser = vec![false; n as usize];
		for i in 0..edges.len() {
			loser[edges[i][1] as usize] = true;
		}
		let mut ans = -1;
		for (i, &result) in loser.iter().enumerate() {
			if result {
				continue;
			}
			if ans != -1 {
				return -1;
			}
			ans = i as i32;
		}
		ans
	}
}