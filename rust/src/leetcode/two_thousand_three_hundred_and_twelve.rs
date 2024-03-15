pub struct Solution;


impl Solution {
	pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
		let m = m as usize;
		let n = n as usize;
		let mut f = vec![vec![0; n + 1]; m + 1];
		for p in &prices {
			f[p[0] as usize][p[1] as usize] = p[2] as i64;
		}
		for i in 1..=m {
			for j in 1..=n {
				for k in 1..=j / 2 {
					f[i][j] = f[i][j].max(f[i][k] + f[i][j - k]);
				}
				for k in 1..=i / 2 {
					f[i][j] = f[i][j].max(f[k][j] + f[i - k][j]);
				}
			}
		}


		f[m][n]
	}
}
