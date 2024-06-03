pub struct Solution;

impl Solution {
	pub fn distribute_candies(mut candies: i32, n: i32) -> Vec<i32> {
		let n = n as usize;
		let mut ans = vec![0; n];
		let mut i = 1;
		while candies > 0 {
			ans[(i - 1) as usize % n] += i.min(candies);
			candies -= i;
			i += 1;
		}
		ans
	}
}