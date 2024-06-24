pub struct Solution;

impl Solution {
	pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
		let n = nums.len();
		let mut ans = vec![-1; n];
		let mut st = vec![];
		for i in (0..2 * n).rev() {
			let x = nums[i % n];
			while let Some(&top) = st.last() {
				if x < top {
					break;
				}
				st.pop();
			}
			if i < n && !st.is_empty() {
				ans[i] = *st.last().unwrap();
			}
			st.push(x);
		}
		ans
	}
}