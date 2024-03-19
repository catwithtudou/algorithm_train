pub struct Solution;

impl Solution {
	pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
		let n = nums.len();
		let mut left = vec![-1; n];
		let mut st = Vec::new();

		for (i, &x) in nums.iter().enumerate() {
			while !st.is_empty() && x <= nums[*st.last().unwrap()] {
				st.pop();
			}
			if let Some(&j) = st.last() {
				left[i] = j as i32;
			}
			st.push(i);
		}


		let mut right = vec![n as i32; n];
		st.clear();
		for (i, &x) in nums.iter().enumerate().rev() {
			while !st.is_empty() && x <= nums[*st.last().unwrap()] {
				st.pop();
			}
			if let Some(&j) = st.last() {
				right[i] = j as i32;
			}
			st.push(i);
		}


		let mut ans = 0;
		for i in 0..n {
			let h = nums[i];
			let l = left[i];
			let r = right[i];
			if l < k && k < r {
				ans = ans.max(h * (r - l - 1));
			}
		}


		ans
	}
}