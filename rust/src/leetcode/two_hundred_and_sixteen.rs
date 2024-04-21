pub struct Solution;


impl Solution {
	pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
		let mut path = Vec::new();
		let mut ans = Vec::new();

		fn dfs(i: i32, t: i32, k: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
			let d = k - path.len() as i32;

			if t < 0 || t > (i + i - d + 1) * d / 2 {
				return;
			}

			if d == 0 {
				ans.push(path.clone());
				return;
			}

			if i > d {
				dfs(i - 1, t, k, path, ans);
			}


			path.push(i);
			dfs(i - 1, t - i, k, path, ans);
			path.pop();
		}

		dfs(9, n, k, &mut path, &mut ans);
		ans
	}
}