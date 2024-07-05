pub struct Solution;


impl Solution {
	pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		for j in 0..matrix[0].len() {
			let mx = matrix.iter().map(|x| x[j]).max().unwrap();
			for row in matrix.iter_mut() {
				if row[j] == -1 {
					row[j] = mx;
				}
			}
		}
		matrix
	}
}