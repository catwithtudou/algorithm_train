pub struct Solution;


impl Solution {
	pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let (m, n) = (mat.len() as i32, mat[0].len() as i32);
		let mut arr = vec![0; m.min(n) as usize];
		for k in 1 - n..m {
			let left = k.max(0) as usize;
			let right = (k + n).min(m) as usize;
			for i in left..right {
				arr[i - left] = mat[i][(i as i32 - k) as usize]
			}
			arr[..right - left].sort_unstable();
			for i in left..right {
				mat[i][(i as i32 - k) as usize] = arr[i - left]
			}
		}
		mat
	}
}