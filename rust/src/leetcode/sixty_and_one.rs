pub struct Solution;

impl Solution {
	pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
		let (mut ans, mut same) = (0, 0);
		for i in 1..temperature_a.len() {
			if temperature_a[i - 1].cmp(&temperature_a[i]) == temperature_b[i - 1].cmp(&temperature_b[i]) {
				same += 1;
				ans = ans.max(same);
			} else {
				same = 0;
			}
		}
		ans
	}
}