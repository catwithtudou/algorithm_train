pub struct Solution;


impl Solution {
	pub fn maximum_sum(arr: Vec<i32>) -> i32 {
		let mut ans = i32::MIN;
		let mut f = vec![[i32::MIN / 2, i32::MIN / 2]; arr.len() + 1];
		for (i, &x) in arr.iter().enumerate() {
			f[i + 1][0] = 0.max(f[i][0]) + x;
			f[i + 1][1] = (f[i][1] + x).max(f[i][0]);
			ans = ans.max(f[i + 1][0]).max(f[i + 1][1]);
		}
		ans
	}
}