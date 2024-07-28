pub struct Solution;

impl Solution {
	pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
		let n = positions.len();
		let mut heights = vec![0; n];

		for i in 0..n {
			let p = &positions[i];
			let (left1, right1) = (p[0], p[1] + p[0] - 1);
			heights[i] = p[1];
			for j in 0..i {
				let q = &positions[j];
				let (left2, right2) = (q[0], q[1] + q[0] - 1);
				if right1 >= left2 && right2 >= left1 {
					heights[i] = heights[i].max(heights[j] + p[1]);
				}
			}
		}

		for i in 1..n {
			heights[i] = heights[i].max(heights[i - 1]);
		}
		heights
	}
}